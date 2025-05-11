use crate::utils::SerialDevice;

use std::env;
use std::sync::{Arc, Mutex};

mod installation;
mod lights;
mod robots;
mod sparklings;
mod utils;

mod config;
use config::{Config, CONFIG};

#[tokio::main]
async fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    eprintln!("Usage: cargo run <command> [subcommands...]");
    return;
  }

  let mut debug = true;
  let mut dry_run = true;

  let mut args = env::args().collect::<Vec<_>>();

  // Look for global flags and remove them from `args`
  args.retain(|arg| match arg.as_str() {
    "--debug" => {
      debug = true;
      false
    }
    "--no-debug" => {
      debug = false;
      false
    }
    "--dry-run" => {
      dry_run = true;
      false
    }
    "--no-dry-run" => {
      dry_run = false;
      false
    }
    _ => true,
  });

  // Initialize global config
  CONFIG
    .set(Config { debug, dry_run })
    .expect("Config already set");

  println!("Selected config: {:?}", CONFIG.get().unwrap());

  match args[1].as_str() {
    "lights" | "l" => handle_lights(&args).await,
    "sparklings" | "s" => handle_sparklings(&args).await,
    "robots" | "r" => handle_robots(&args).await,
    "installation" | "i" => handle_installation(&args).await,
    _ => eprintln!("Unknown command: {}", args[1]),
  }
}

async fn handle_robots(args: &[String]) {
  if args.len() < 4 {
    eprintln!("Usage: cargo run robots <ID> <init/move> [<pos> <speed>]");
    return;
  }

  let id = &args[2];
  let command = &args[3];

  match command.as_str() {
    "init" => {
      println!("Initializing robot with ID: {}", id);
      let robot_manager = robots::RobotManager::new().await;
      let robot = match id.as_str() {
        "1" => robot_manager.robot_b,
        "2" => robot_manager.robot_c,
        "3" => robot_manager.robot_a,
        "4" => robot_manager.robot_d,
        _ => panic!("Invalid Robot ID: {}", id),
      };
      // let robot = robots::create(id);
      robot.init().await;
    }
    "move" => {
      if args.len() < 6 {
        eprintln!("Usage: cargo run robots <ID> move <pos> <speed>");
        return;
      }
      let pos_str = &args[4];
      let speed_str = &args[5];
      println!(
        "Moving robot ID: {} to position: {} with speed: {}",
        id, pos_str, speed_str
      );
      let robot_manager = robots::RobotManager::new().await;
      let robot = match id.as_str() {
        "1" => robot_manager.robot_b,
        "2" => robot_manager.robot_c,
        "3" => robot_manager.robot_a,
        "4" => robot_manager.robot_d,
        _ => panic!("Invalid Robot ID: {}", id),
      };
      // let robot = robots::create(id);
      let pos: f64 = pos_str.parse().expect("Invalid position value");
      let speed: f64 = speed_str.parse().expect("Invalid speed value");
      robot.set_position(pos, speed).await;
    }
    _ => eprintln!("Unknown robots subcommand: {}", command),
  }

  std::process::exit(0);
}

async fn handle_lights(args: &[String]) {
  if args.len() < 4 {
    eprintln!("Usage: cargo run lights <ID> <ON/OFF>");
    return;
  }

  let id = &args[2];
  let state = &args[3];

  let serial_device: Arc<Mutex<dyn SerialDevice>> =
    if config::get(config::ConfigParam::DRYRUN) {
      Arc::new(Mutex::new(
        utils::MockSerialDevice::new(lights::LIGHT_SERIAL_PORT_NAME, lights::LIGHT_SERIAL_BAUD)
          .expect("Cannot initialize MockSerialDevice"),
      ))
    } else {
      let real =
        utils::RealSerialDevice::new(lights::LIGHT_SERIAL_PORT_NAME, lights::LIGHT_SERIAL_BAUD)
          .await
          .expect("Cannot initialize RealSerialDevice");
      Arc::new(Mutex::new(real))
    };
  match state.as_str() {
    "on" => {
      println!("Turning light {} for ID: {}", state, id);
      let mut light = lights::create(id, Arc::clone(&serial_device)).await;
      light.turn_on();
    }
    "off" => {
      println!("Turning light {} for ID: {}", state, id);
      let mut light = lights::create(id, Arc::clone(&serial_device)).await;
      light.turn_off();
    }
    _ => eprintln!("Invalid state for light: {}", state),
  }
}

async fn handle_sparklings(args: &[String]) {
  if args.len() < 4 {
    eprintln!("Usage: cargo run sparklings <ID> <ON/OFF>");
    return;
  }

  let id = &args[2];
  let state = &args[3];

  match state.as_str() {
    "on" => {
      println!("Turning sparkling {} for ID: {}", state, id);
      let sparkling = sparklings::create(id);
      sparkling.turn_on().await;
    }
    "off" => {
      println!("Turning sparkling {} for ID: {}", state, id);
      let sparkling = sparklings::create(id);
      sparkling.turn_off().await;
    }
    _ => eprintln!("Invalid state for spark: {}", state),
  }
}

async fn handle_installation(args: &[String]) {
  if args.len() < 3 {
    eprintln!("Usage: cargo run installation <start/stop>");
    return;
  }

  match args[2].as_str() {
    "start" => {
      installation::start().await;
      println!("Starting installation");
    }
    "stop" => println!("Stopping installation"),
    _ => eprintln!("Unknown installation subcommand: {}", args[2]),
  }
}
