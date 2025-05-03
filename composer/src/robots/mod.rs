use rand::Rng;
use tokio;

use crate::utils;
use crate::DRY_RUN;

const SCANNING_POSITION: f64 = 0.8;
const SYNCING_POSITION: f64 = 0.2;

const BUFFERING_TIME: u64 = 1000 * 60 * 20;
const SCANNING_TIME: u64 = 1000 * 60 * 1;
const SYNCING_TIME: u64 = 1000 * 60 * 1;

pub struct RobotManager {
  pub robot_a: Robot,
  pub robot_b: Robot,
  pub robot_c: Robot,
  pub robot_d: Robot,
}

impl RobotManager {
  pub fn new() -> Self {
    let robot_manager = RobotManager {
      robot_a: Robot::new(0, "A", 30),
      robot_b: Robot::new(1, "B", 29),
      robot_c: Robot::new(2, "C", 31),
      robot_d: Robot::new(3, "D", 28),
    };
    robot_manager.initialize_all();
    return robot_manager;
  }
  pub fn initialize_all(&self) {
    self.robot_a.init();
    self.robot_b.init();
    self.robot_c.init();
    self.robot_d.init();
  }
  pub async fn start_buffering(&mut self) {
    tokio::join!(
      countdown(BUFFERING_TIME),
      self.robot_a.start_buffering(),
      self.robot_b.start_buffering(),
      self.robot_c.start_buffering(),
      self.robot_d.start_buffering(),
    );
  }
  pub async fn start_scanning(&mut self) {
    tokio::join!(
      self.robot_a.start_scanning(),
      self.robot_b.start_scanning(),
      self.robot_c.start_scanning(),
      self.robot_d.start_scanning(),
    );
  }
  pub async fn start_syncing(&mut self) {
    tokio::join!(
      self.robot_a.start_syncing(),
      self.robot_b.start_syncing(),
      self.robot_c.start_syncing(),
      self.robot_d.start_syncing(),
    );
  }
}

#[derive(Debug, PartialEq)]
pub enum RobotState {
  Buffering,
  Scanning,
  Syncing,
}

pub struct Robot {
  id: u8,
  name: &'static str,
  init_sleep_seconds: u64,
  state: RobotState,
}

impl Robot {
  fn new(id: u8, name: &'static str, init_sleep_seconds: u64) -> Self {
    Robot {
      id,
      name,
      init_sleep_seconds,
      state: RobotState::Buffering,
    }
  }
  fn init(&self) {
    if DRY_RUN {
      utils::print_dry_run("Invoked robot init script");
      utils::sleep(self.init_sleep_seconds);
      self.print();
      return;
    }
    utils::invoke_script(&utils::ScriptName::RobotInit, &[&self.name]);
  }
  pub async fn start_buffering(&mut self) {
    let random_sleep_time = random_integer_value(10, 60);
    let start_sleep_time = 1000 * random_sleep_time;
    utils::sleep(start_sleep_time);

    self.stop();
    self.state = RobotState::Buffering;
    loop {
      if self.state != RobotState::Buffering {
        break;
      }
      let random_position = random_normal_value();
      let random_speed = random_normal_value();
      self.set_position(random_position, random_speed);
    }
  }
  pub async fn start_scanning(&mut self) {
    let random_sleep_time = random_integer_value(10, 60);
    let start_sleep_time = 1000 * random_sleep_time;
    utils::sleep(start_sleep_time);

    self.stop();

    self.state = RobotState::Scanning;

    let random_speed = random_normal_value();
    self.set_position(SCANNING_POSITION, random_speed);

    utils::sleep(SCANNING_TIME);
  }
  pub async fn start_syncing(&mut self) {
    let random_sleep_time = random_integer_value(10, 60);
    let start_sleep_time = 1000 * random_sleep_time;
    utils::sleep(start_sleep_time);

    self.stop();

    self.state = RobotState::Syncing;

    let random_speed = random_normal_value();
    self.set_position(SYNCING_POSITION, random_speed);

    utils::sleep(SYNCING_TIME);
  }

  pub fn stop(&self) -> f64 {
    let position = self.get_position();
    let position_f64 = position.parse::<f64>().unwrap();
    self.set_position(position_f64, 0.9);
    return position_f64;
  }

  pub fn set_position(&self, position: f64, speed: f64) {
    if DRY_RUN {
      utils::print_dry_run("Invoked robot set position script");
      return;
    }
    let current_position = self.stop();
    utils::invoke_script(
      &utils::ScriptName::RobotSetPosition,
      &[self.name, position.to_string().as_str()],
    );
    let time = resolve_time(&current_position, &position, &speed);
    utils::sleep(time);
  }

  pub fn get_position(&self) -> String {
    if DRY_RUN {
      utils::print_dry_run("Invoked robot get position script");
      return String::from("[FAKE];0;0");
    }
    crate::log_enter!("invoke_get_position", self.name);
    let response =
      utils::invoke_script(&utils::ScriptName::RobotGetPosition, &[self.name]);
    let unwrapped = response.unwrap();
    print!("Get position returned: ------> {}", unwrapped);
    crate::log_exit!("invoke_get_position", self.name);
    return unwrapped;
  }

  fn print(&self) {
    println!("{} {} {}", self.id, self.name, self.init_sleep_seconds);
  }
}

fn resolve_time(current_position: &f64, position: &f64, speed: &f64) -> u64 {
  if *speed <= 0.0 {
    panic!("Speed must be greater than zero");
  }
  let distance = (position - current_position).abs();
  let time = distance / speed;
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

async fn countdown(seconds: u64) {
  for i in (1..=seconds).rev() {
    println!("{} seconds remaining...", i);
    utils::sleep(1000);
  }
}
