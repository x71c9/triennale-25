use crate::{lights, robots, sparklings};

struct Composer {
  robot_manager: robots::RobotManager,
  light_manager: lights::LightManager,
  sparkling_manager: sparklings::SparklingManager,
}

impl Composer {
  async fn new() -> Self {
    println!("Initializing composer...");
    let composer: Composer = Composer {
      robot_manager: robots::RobotManager::new().await,
      light_manager: lights::LightManager::new().await,
      sparkling_manager: sparklings::SparklingManager::new().await,
    };
    println!("Composer initialized");
    return composer;
  }
  async fn start(&mut self) {
    println!("Starting composer...");
    loop {
      self.start_buffering().await;
      self.start_scanning().await;
      self.start_buffering().await;
      self.start_syncing().await;
    }
  }
  // async fn move_robot(&mut self){
  //   self.robot_manager.robot_a.set_position(1.0, 1.0).await;
  // }
  async fn start_buffering(&mut self) {
    println!("BUFFERING STATE INITIATED...");
    self.robot_manager.start_buffering().await;
  }
  async fn start_scanning(&mut self) {
    println!("SCANNING STATE INITIATED...");
    self.robot_manager.start_scanning().await;
    self.sparkling_manager.run_sparkling().await;
  }
  async fn start_syncing(&mut self) {
    println!("SYNCING STATE INITIATED...");
    self.robot_manager.start_syncing().await;
    self.light_manager.regulate_light().await;
  }
}

pub async fn start() {
  std::panic::set_hook(Box::new(|info| {
    eprintln!("[!] Panic occurred: {info}");
    std::process::exit(1);
  }));

  let mut composer = Composer::new().await;
  composer.start().await;
}
