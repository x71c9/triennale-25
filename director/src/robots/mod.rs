use rand::Rng;
use rosc::{encoder, OscMessage, OscPacket, OscType};
use std::fmt;
use std::net::UdpSocket;
use std::sync::Arc;
use tokio::sync::RwLock;

use tokio;

use crate::config::{self, ConfigParam};
use crate::utils;

// const SERVICE_ADDRESS: &'static str = "127.0.0.1:5000";
const SERVICE_ADDRESS: &'static str = "255.255.255.255:6666";

const SCANNING_POSITION: f64 = 4.8;
const SYNCING_POSITION: f64 = 0.8;

const BUFFERING_TIME_MS: u64 = 1000 * 60 * 2;
const SCANNING_TIME_MS: u64 = 1000 * 60 * 1;
const SYNCING_TIME_MS: u64 = 1000 * 60 * 1;

const ROBOT_A_INIT_TIME_MS: u64 = 1000 * 5;
const ROBOT_B_INIT_TIME_MS: u64 = 1000 * 5;
const ROBOT_C_INIT_TIME_MS: u64 = 1000 * 5;
const ROBOT_D_INIT_TIME_MS: u64 = 1000 * 5;

const ROBOT_A_CONSTANT_TIME_MS: u64 = 1000 * 93;
const ROBOT_B_CONSTANT_TIME_MS: u64 = 1000 * 86;
const ROBOT_C_CONSTANT_TIME_MS: u64 = 1000 * 90;
const ROBOT_D_CONSTANT_TIME_MS: u64 = 1000 * 93;

const POSITION_INTERVAL_MS: u64 = 100;

const BUFFERING_MIN_DELAY_MS: u64 = 1000 * 10;
const BUFFERING_MAX_DELAY_MS: u64 = 1000 * 60;

const SCANNING_MIN_DELAY_MS: u64 = 1000 * 10;
const SCANNING_MAX_DELAY_MS: u64 = 1000 * 60;

const SYNCING_MIN_DELAY_MS: u64 = 1000 * 10;
const SYNCING_MAX_DELAY_MS: u64 = 1000 * 60;

pub struct RobotManager {
  pub robot_a: Arc<Robot>,
  pub robot_b: Arc<Robot>,
  pub robot_c: Arc<Robot>,
  pub robot_d: Arc<Robot>,
}

impl RobotManager {
  pub async fn new() -> Self {
    crate::log_enter!("---- RobotManager new", "");
    let robot_manager = RobotManager {
      robot_a: Arc::new(Robot::new(
        0,
        "A",
        ROBOT_A_INIT_TIME_MS,
        ROBOT_A_CONSTANT_TIME_MS,
      )),
      robot_b: Arc::new(Robot::new(
        1,
        "B",
        ROBOT_B_INIT_TIME_MS,
        ROBOT_B_CONSTANT_TIME_MS,
      )),
      robot_c: Arc::new(Robot::new(
        2,
        "C",
        ROBOT_C_INIT_TIME_MS,
        ROBOT_C_CONSTANT_TIME_MS,
      )),
      robot_d: Arc::new(Robot::new(
        3,
        "D",
        ROBOT_D_INIT_TIME_MS,
        ROBOT_D_CONSTANT_TIME_MS,
      )),
    };
    // robot_manager.initialize_all().await;
    let ra = Arc::clone(&robot_manager.robot_a);
    let rb = Arc::clone(&robot_manager.robot_b);
    let rc = Arc::clone(&robot_manager.robot_c);
    let rd = Arc::clone(&robot_manager.robot_d);
    tokio::spawn(async move {
      start_service(ra, rb, rc, rd).await;
    });
    crate::log_exit!("---- RobotManager new", "");
    return robot_manager;
  }
  pub async fn initialize_all(&self) {
    crate::log_enter!("RobotManager initialize_all", "");
    self.robot_a.init().await;
    self.robot_b.init().await;
    self.robot_c.init().await;
    self.robot_d.init().await;
    crate::log_exit!("RobotManager initialize_all", "");
  }
  pub async fn start_buffering(&mut self) {
    crate::log_enter!("RobotManager start_buffering", "");
    tokio::join!(
      countdown(BUFFERING_TIME_MS),
      self.robot_a.start_buffering(),
      self.robot_b.start_buffering(),
      self.robot_c.start_buffering(),
      self.robot_d.start_buffering(),
    );
    crate::log_enter!("RobotManager stop_buffering", "");
  }
  pub async fn start_scanning(&mut self) {
    let delay_a = get_scanning_delay();
    let delay_b = get_scanning_delay();
    let delay_c = get_scanning_delay();
    let delay_d = get_scanning_delay();
    let max_delay = get_max(vec![delay_a, delay_b, delay_c, delay_d]);
    println!("Start scanning max delay in milliseconds: {}", max_delay);
    tokio::join!(
      countdown(SCANNING_TIME_MS + max_delay),
      self.robot_a.start_scanning(delay_a),
      self.robot_b.start_scanning(delay_b),
      self.robot_c.start_scanning(delay_c),
      self.robot_d.start_scanning(delay_d),
    );
  }
  pub async fn start_syncing(&mut self) {
    let delay_a = get_syncing_delay();
    let delay_b = get_syncing_delay();
    let delay_c = get_syncing_delay();
    let delay_d = get_syncing_delay();
    let max_delay = get_max(vec![delay_a, delay_b, delay_c, delay_d]);
    println!("Start syncing max delay in milliseconds: {}", max_delay);
    tokio::join!(
      countdown(SYNCING_TIME_MS + max_delay),
      self.robot_a.start_syncing(delay_a),
      self.robot_b.start_syncing(delay_b),
      self.robot_c.start_syncing(delay_c),
      self.robot_d.start_syncing(delay_d),
    );
  }
}

#[derive(Debug, PartialEq)]
pub enum RobotState {
  Buffering,
  Scanning,
  Syncing,
}

impl fmt::Display for RobotState {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      RobotState::Buffering => write!(f, "BUFFERING"),
      RobotState::Scanning => write!(f, "SCANNING"),
      RobotState::Syncing => write!(f, "SYNCING"),
    }
  }
}

pub struct Robot {
  id: u8,
  name: &'static str,
  init_time: u64,
  state: RwLock<RobotState>,
  position: RwLock<f64>,
  speed_constant: u64,
}

impl Robot {
  fn new(
    id: u8,
    name: &'static str,
    init_time: u64,
    speed_constant: u64,
  ) -> Self {
    crate::log_enter!("Robot new", id);
    let robot = Robot {
      id,
      name,
      init_time,
      state: RwLock::new(RobotState::Buffering),
      position: RwLock::new(0.0),
      speed_constant,
    };
    crate::log_exit!("Robot new", id);
    return robot;
  }

  pub async fn init(&self) {
    crate::log_enter!("Robot init", &self.id);
    if config::get(ConfigParam::DRYRUN) {
      utils::print_dry_run("Invoked robot init script");
      utils::sleep(self.init_time, "Robot init").await;
      self.print();
      crate::log_exit!("Robot init", &self.id);
      return;
    }
    utils::invoke_script(&utils::ScriptName::RobotInit, &[&self.name]);
    // utils::sleep(self.init_time, "Robot init").await;
    crate::log_exit!("Robot init", &self.id);
  }

  pub async fn start_buffering(self: &Arc<Self>) {
    // TODO remove
    if self.id > 0 {
      return;
    }
    crate::log_enter!("Robot start_buffering", self.id);
    // self.stop().await;
    {
      let mut state = self.state.write().await;
      *state = RobotState::Buffering;
    }
    let start_time = std::time::Instant::now();
    loop {
      if *self.state.read().await != RobotState::Buffering {
        break;
      }
      if start_time.elapsed().as_secs() >= BUFFERING_TIME_MS / 1000 {
        break;
      }
      let delay = get_buffering_delay();
      utils::sleep(delay, "Robot start_buffering").await;
      let random_position = random_normal_value() * 5.0;
      let random_speed = random_normal_value();
      self.set_position(random_position, random_speed).await;
    }
    crate::log_exit!("Robot start_buffering", self.id);
  }

  pub async fn start_scanning(self: &Arc<Self>, delay: u64) {
    crate::log_enter!("Robot start_scanning", self.id);
    // self.stop().await;
    {
      let mut state = self.state.write().await;
      *state = RobotState::Scanning;
    }
    utils::sleep(delay, "Robot start_scanning").await;
    let random_speed = random_normal_value();
    self.set_position(SCANNING_POSITION, random_speed).await;
    utils::sleep(SCANNING_TIME_MS, "Robot start_scanning SCANNING_TIME").await;
    crate::log_exit!("Robot start_scanning", self.id);
  }

  pub async fn start_syncing(self: &Arc<Self>, delay: u64) {
    crate::log_enter!("Robot start_syncing", self.id);
    // self.stop().await;
    {
      let mut state = self.state.write().await;
      *state = RobotState::Syncing;
    }
    utils::sleep(delay, "Robot start_syncing").await;
    let random_speed = random_normal_value();
    self.set_position(SYNCING_POSITION, random_speed).await;
    utils::sleep(SYNCING_TIME_MS, "Robot start_syncing SYNCYING_TIME").await;
    crate::log_exit!("Robot start_syncing", self.id);
  }

  // pub async fn stop(self: &Arc<Self>) -> f64 {
  //   crate::log_enter!("Robot stop", self.id);
  //   let current_position = self.get_position().await;
  //   self.set_position(current_position, 1.0).await;
  //   crate::log_exit!("Robot stop", self.id);
  //   return current_position;
  // }

  pub async fn set_position(self: &Arc<Self>, pos: f64, speed: f64) {
    crate::log_enter!("Robot set_position", pos);
    // let current_position = *self.position.read().await;
    let current_position = self.get_real_position().await;
    println!("Current position is {}", current_position);
    if config::get(ConfigParam::DRYRUN) {
      utils::print_dry_run(
        format!("Invoked robot set position script {} {}", pos, speed).as_str(),
      );
    } else {
      utils::invoke_script(
        &utils::ScriptName::RobotSetPosition,
        &[self.name, &pos.to_string(), &speed.to_string()],
      );
    }
    let mapped_position = map_position(pos);
    let time = resolve_time_ms(
      &current_position,
      &mapped_position,
      &speed,
      &(self.speed_constant as f64),
    );
    println!("Resolved time {}", time);
    let delta = mapped_position - current_position;
    println!("Resolved delta {}", delta);
    let steps = ((time / POSITION_INTERVAL_MS) as f64).ceil() as usize;
    println!("Resolved steps {}", steps);
    let step_size = delta / steps as f64;
    println!("Resolved steps_size {}", step_size);
    for _ in 0..steps {
      {
        let mut p = self.position.write().await;
        *p += step_size;
      }
      utils::sleep_silent(POSITION_INTERVAL_MS).await;
    }
    println!("Loop stopped");
    let mut p = self.position.write().await;
    *p = mapped_position;
    let after_position = *self.position.read().await;
    println!("Current position is {}", after_position);
    crate::log_exit!("Robot set_position", mapped_position);
  }

  pub async fn get_real_position(&self) -> f64 {
    crate::log_enter!("Robot get_real_position", "");
    let pos = if config::get(ConfigParam::DRYRUN) {
      utils::print_dry_run(
        format!("Invoked robot get position script").as_str(),
      );
      *self.position.read().await
    } else {
      let response = utils::invoke_script(
        &utils::ScriptName::RobotGetPosition,
        &[self.name],
      );
      let unwrapped = response.expect("");
      let last_line = unwrapped
        .lines()
        .filter(|line| !line.trim().is_empty())
        .last()
        .expect("No lines in script output");
      println!("Get real position response:\n{}", last_line);
      let number_value: f64 = last_line.parse().expect("Cannot parse real pos");
      number_value
    };
    let mapped_position = map_position(pos);
    println!("Current position mapped is {}", mapped_position);
    crate::log_exit!("Robot get_real_position", mapped_position);
    return mapped_position;
  }

  // pub async fn get_position(&self) -> f64 {
  //   crate::log_enter!("Robot get_position", self.id);
  //   let pos = *self.position.read().await;
  //   crate::log_exit!("Robot get_position RESPONSE: ", pos);
  //   return pos;
  // }

  // fn get_position(&self) -> f64 {
  //   return self.position;
  //   // if DRYRUN {
  //   //   utils::print_dry_run("Invoked robot get position script");
  //   //   return String::from("[FAKE];0;0");
  //   // }
  //   // crate::log_enter!("invoke_get_position", self.name);
  //   // let response =
  //   //   utils::invoke_script(&utils::ScriptName::RobotGetPosition, &[self.name]);
  //   // let unwrapped = response.unwrap();
  //   // print!("Get position returned: ------> {}", unwrapped);
  //   // crate::log_exit!("invoke_get_position", self.name);
  //   // return unwrapped;
  // }

  fn print(&self) {
    println!("ROBOT {} {} {}", self.id, self.name, self.init_time);
  }
}

fn resolve_time_ms(
  current_position: &f64,
  position: &f64,
  speed: &f64,
  k: &f64,
) -> u64 {
  println!(
    "Invoked resolve_time_ms with current: {}, position: {}, speed: {}, k: {}",
    current_position, position, speed, k
  );
  if *speed <= 0.0 {
    panic!("Speed must be greater than zero");
  }
  let distance = (position - current_position).abs();
  println!("Distance: {}", distance);
  let time = (distance / speed) * k;
  println!("Time: {}", time);
  time.ceil() as u64 // Round up to ensure enough time to reach
}

fn random_normal_value() -> f64 {
  let mut rng = rand::thread_rng();
  let random_value: f64 = rng.gen_range(0.1..0.9);
  return random_value;
}

fn random_integer_value(from: u64, to: u64) -> u64 {
  let mut rng = rand::thread_rng();
  let random_value: u64 = rng.gen_range(from..to);
  return random_value;
}

fn get_max(values: Vec<u64>) -> u64 {
  let values_string = format!("Cannot find maximun of values {:?}", values);
  let max = values.iter().copied().max().expect(&values_string);
  return max;
}

async fn countdown(milliseconds: u64) {
  println!("Srarting countdown milliseconds {}", milliseconds);
  for remaining in (1..=milliseconds / 1000).rev() {
    let minutes = remaining / 60;
    let seconds = remaining % 60;
    println!(
      "T-00:{:02}:{:02} remaining to next process...",
      minutes, seconds
    );
    utils::sleep_silent(1000).await;
  }
}

async fn start_service(
  robot_a: Arc<Robot>,
  robot_b: Arc<Robot>,
  robot_c: Arc<Robot>,
  robot_d: Arc<Robot>,
) {
  println!("Starting Robot Service...");
  let socket = UdpSocket::bind("0.0.0.0:0").expect("could not bind socket");
  socket
    .set_broadcast(true)
    .expect("could not enable broadcast");

  loop {
    let positions = vec![
      *robot_a.position.read().await,
      *robot_b.position.read().await,
      *robot_c.position.read().await,
      *robot_d.position.read().await,
    ];
    let state = robot_a.state.read().await;

    let mut msg_string = positions
      .iter()
      .map(|f| format!("{:.10}", f))
      .collect::<Vec<_>>()
      .join(",");
    msg_string.push_str(&format!(",{}", state));

    let msg = OscMessage {
      addr: "/robots".into(),
      args: vec![OscType::String(msg_string)],
    };

    let packet = OscPacket::Message(msg);
    let buf = encoder::encode(&packet).unwrap();
    socket.send_to(&buf, SERVICE_ADDRESS).unwrap();

    utils::sleep_silent(POSITION_INTERVAL_MS).await;
  }
}

fn get_buffering_delay() -> u64 {
  let delay = random_integer_value(
    BUFFERING_MIN_DELAY_MS / 1000,
    BUFFERING_MAX_DELAY_MS / 1000,
  ) * 1000;
  return delay;
}

fn get_scanning_delay() -> u64 {
  let delay = random_integer_value(
    SCANNING_MIN_DELAY_MS / 1000,
    SCANNING_MAX_DELAY_MS / 1000,
  ) * 1000;
  return delay;
}

fn get_syncing_delay() -> u64 {
  let delay = random_integer_value(
    SYNCING_MIN_DELAY_MS / 1000,
    SYNCING_MAX_DELAY_MS / 1000,
  ) * 1000;
  return delay;
}

// pub fn create(id: &str) -> Arc<Robot> {
//   match id {
//     "1" => Arc::new(Robot::new(
//       2,
//       "B",
//       ROBOT_B_INIT_TIME_MS,
//       ROBOT_B_CONSTANT_TIME_MS,
//     )),
//     "2" => Arc::new(Robot::new(
//       3,
//       "C",
//       ROBOT_C_INIT_TIME_MS,
//       ROBOT_C_CONSTANT_TIME_MS,
//     )),
//     "3" => Arc::new(Robot::new(
//       1,
//       "A",
//       ROBOT_A_INIT_TIME_MS,
//       ROBOT_A_CONSTANT_TIME_MS,
//     )),
//     "4" => Arc::new(Robot::new(
//       4,
//       "D",
//       ROBOT_D_INIT_TIME_MS,
//       ROBOT_D_CONSTANT_TIME_MS,
//     )),
//     _ => {
//       panic!("Invalid Robot ID. Possible value [1-4]");
//     }
//   }
// }

fn map_position(value: f64) -> f64 {
  (value / 5.0).clamp(0.0, 1.0)
}
