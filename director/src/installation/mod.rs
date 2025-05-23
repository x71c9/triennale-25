use crate::{lights, robots, sparklings};

struct Director {
  robot_manager: robots::RobotManager,
  light_manager: lights::LightManager,
  sparkling_manager: sparklings::SparklingManager,
}

impl Director {
  async fn new() -> Self {
    println!("Initializing director...");
    let director: Director = Director {
      robot_manager: robots::RobotManager::new().await,
      light_manager: lights::LightManager::new().await,
      sparkling_manager: sparklings::SparklingManager::new().await,
    };
    director.robot_manager.initialize_all().await;
    println!("Director initialized");
    return director;
  }
  async fn start(&mut self) {
    println!("Starting director...");
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

  let mut director = Director::new().await;
  director.start().await;
}
