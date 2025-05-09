use std::env;

mod installation;
mod lights;
mod robots;
mod sparklings;
mod utils;

pub const DEBUG: bool = true;
pub const DRY_RUN: bool = true;
pub const NARROW: bool = true;

#[tokio::main]
async fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    eprintln!("Usage: cargo run <command> [subcommands...]");
    return;
  }

  match args[1].as_str() {
    "robots" | "r" => handle_robots(&args).await,
    "lights" | "l" => handle_lights(&args).await,
    "sparklings" | "s" => handle_sparklings(&args).await,
    "installation" | "i" => handle_installation(&args).await,
    _ => eprintln!("Unknown command: {}", args[1]),
  }
}

async fn handle_robots(args: &[String]) {
  if args.len() < 4 {
    eprintln!("Usage: cargo run robots <init/move> <ID> [<pos> <speed>]");
    return;
  }
  match args[2].as_str() {
    "init" => {
      let id = &args[3];
      println!("Initializing robot with ID: {}", id);
      let robot = robots::create(id);
      robot.init().await;
    }
    "move" => {
      if args.len() < 6 {
        eprintln!("Usage: cargo run robots move <ID> <pos> <speed>");
        return;
      }
      let id = &args[3];
      let pos = &args[4];
      let speed = &args[5];
      println!(
        "Moving robot ID: {} to position: {} with speed: {}",
        id, pos, speed
      );
      let robot = robots::create(id);
      let pos: f64 = args[4].parse().expect("Invalid position value");
      let speed: f64 = args[5].parse().expect("Invalid speed value");
      robot.set_position(pos, speed).await;
    }
    _ => eprintln!("Unknown robots subcommand: {}", args[2]),
  }
}

async fn handle_lights(args: &[String]) {
  if args.len() < 4 {
    eprintln!("Usage: cargo run light <ON/OFF> <ID>");
    return;
  }

  let state = &args[2];
  let id = &args[3];

  match state.as_str() {
    "ON" => {
      println!("Turning light {} for ID: {}", state, id);
      let mut light = lights::create(id);
      light.turn_on();
    },
    "OFF" => {
      println!("Turning light {} for ID: {}", state, id);
      let mut light = lights::create(id);
      light.turn_off();
    },
    _ => eprintln!("Invalid state for light: {}", state),
  }
}

async fn handle_sparklings(args: &[String]) {
  if args.len() < 4 {
    eprintln!("Usage: cargo run spark <ON/OFF> <ID>");
    return;
  }

  let state = &args[2];
  let id = &args[3];

  match state.as_str() {
    "ON" => {
      println!("Turning sparkling {} for ID: {}", state, id);
      let sparkling = sparklings::create(id);
      sparkling.turn_on();
    },
    "OFF" => {
      println!("Turning sparkling {} for ID: {}", state, id);
      let sparkling = sparklings::create(id);
      sparkling.turn_off();
    },
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
