use crate::utils;
use crate::DRY_RUN;

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
}

pub struct Robot {
  id: u8,
  name: &'static str,
  init_sleep_seconds: u64,
}

impl Robot {
  fn new(id: u8, name: &'static str, init_sleep_seconds: u64) -> Self {
    Robot {
      id,
      name,
      init_sleep_seconds,
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
  pub fn set_position(&self, position: f64) {
    if DRY_RUN {
      utils::print_dry_run("Invoked robot set position script");
      return;
    }
    utils::invoke_script(
      &utils::ScriptName::RobotSetPosition,
      &[self.name, position.to_string().as_str()],
    );
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
