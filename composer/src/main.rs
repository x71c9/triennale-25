use futures::future;
use tokio::task;

mod lights;
mod robots;
mod services;
mod sparklings;
mod utils;

pub const DEBUG: bool = false;
pub const DRY_RUN: bool = true;

#[tokio::main]
async fn main() {

  std::panic::set_hook(Box::new(|info| {
    eprintln!("[!] Panic occurred: {info}");
    std::process::exit(1);
  }));

  let mut tasks = Vec::new();

  let robotposition_service_task = task::spawn(async move {
    services::robotpositions::send()
      .expect("[!] ERROR: Robotposition service failed");
  });
  tasks.push(robotposition_service_task);

  let robot_manager = robots::RobotManager::new();
  robot_manager.initialize_all();

  let robot_initialization_task = task::spawn(async move {
    // TODO
    println!("Robot async job...");
    robot_manager.robot_a.set_position(0.5);
    robot_manager.robot_b.get_position();
  });
  tasks.push(robot_initialization_task);

  let lights_initialization_task = task::spawn(async move {
    utils::sleep(5000);
    lights::init();
  });
  tasks.push(lights_initialization_task);

  let sparkling_initialization_task = task::spawn(async move {
    utils::sleep(15000);
    sparklings::init();
  });
  tasks.push(sparkling_initialization_task);

  match future::try_join_all(tasks).await {
    Ok(_) => println!("[*] All tasks completed successfully."),
    Err(e) => eprintln!("[!] A task failed: {:?}", e),
  }
}
