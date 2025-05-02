use std::collections::HashMap;
use tokio::task::{self, JoinHandle};

mod lights;
mod robots;
mod services;
mod sparklings;
mod utils;

pub const DEBUG: bool = false;
pub const DRY_RUN: bool = true;

struct Composer {
  robot_manager: robots::RobotManager,
  light_manager: lights::LightManager,
  sparkling_manager: sparklings::SparklingManager,
  tasks: HashMap<String, JoinHandle<()>>,
}

impl Composer {
  fn new() -> Self {
    let mut composer: Composer = Composer {
      robot_manager: robots::RobotManager::new(),
      light_manager: lights::LightManager::new(),
      sparkling_manager: sparklings::SparklingManager::new(),
      tasks: HashMap::new(),
    };
    composer.robot_manager.initialize_all();

    let robotposition_service_task = task::spawn(async {
      services::robotpositions::send()
        .expect("[!] ERROR: Robotposition service failed");
    });
    composer.tasks.insert(
      "robotpositions_service".to_string(),
      robotposition_service_task,
    );
    return composer;
  }
  fn start(&self) {
    loop {
      self.start_buffering();
      utils::sleep(20 * 60); // 20 min

      self.start_scanning();
      utils::sleep(2 * 60); // 2 min

      self.start_buffering();
      utils::sleep(20 * 60); // 20 min

      self.start_syncing();
      utils::sleep(2 * 60); // 2 min
    }
  }
  fn start_buffering(&self) {
    println!("Started buffering...");
    self.robot_manager.start_buffering();
  }
  fn stop_buffering(&self) {
    println!("Stopped buffering");
    self.robot_manager.stop_buffering();
  }
  fn start_scanning(&self) {
    println!("Started buffering...");
    self.stop_buffering();
    self.robot_manager.move_to_scanning_position();
    self.sparkling_manager.run_sparkling();
  }
  fn start_syncing(&self) {
    println!("Started syncing...");
    self.stop_buffering();
    self.robot_manager.move_to_syncing_position();
    self.light_manager.regulate_light();
  }
}

#[tokio::main]
async fn main() {
  std::panic::set_hook(Box::new(|info| {
    eprintln!("[!] Panic occurred: {info}");
    std::process::exit(1);
  }));

  let composer = Composer::new();
  composer.start();
}
