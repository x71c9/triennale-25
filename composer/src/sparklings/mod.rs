use crate::utils::{self, print_dry_run, SerialDevice};
use crate::DRY_RUN;

const SPARKLING_SERIAL_PORT_NAME: &'static str = "/dev/tty.usbmodem14101";
const SPARKLING_SERIAL_BAUD: u32 = 9600;

pub struct SparklingManager {
  pub sparkling_a: Sparkling,
  pub sparkling_b: Sparkling,
  pub sparkling_c: Sparkling,
}

impl SparklingManager {
  pub fn new() -> Self {
    let mut sparkiling_manager: SparklingManager = SparklingManager {
      sparkling_a: Sparkling::new(0, "A"),
      sparkling_b: Sparkling::new(1, "B"),
      sparkling_c: Sparkling::new(2, "C"),
    };
    sparkiling_manager.all_turn_on();
    utils::sleep(5000);
    sparkiling_manager.all_turn_off();
    return sparkiling_manager;
  }
  pub fn run_sparkling(&self){
    // TODO
    self.sparkling_a.run_sparkling();
  }
  pub fn all_turn_on(&mut self) {
    crate::log_enter!("sparkling.all_turn_on", "");
    self.sparkling_a.turn_on();
    self.sparkling_b.turn_on();
    self.sparkling_c.turn_on();
    crate::log_exit!("sparkling.all_turn_on", "");
  }
  pub fn all_turn_off(&mut self) {
    crate::log_enter!("sparkling.all_turn_off", "");
    self.sparkling_a.turn_off();
    self.sparkling_b.turn_off();
    self.sparkling_c.turn_off();
    crate::log_exit!("sparkling.all_turn_off", "");
  }
}

pub struct Sparkling {
  pub id: u8,
  pub name: &'static str,
  pub serial_device: SerialDevice,
}

impl Sparkling {
  pub fn new(id: u8, name: &'static str) -> Self {
    let light: Sparkling = Sparkling {
      id,
      name,
      serial_device: SerialDevice::new(
        SPARKLING_SERIAL_PORT_NAME,
        SPARKLING_SERIAL_BAUD,
      )
      .unwrap(),
    };
    light.print();
    return light;
  }
  pub fn run_sparkling(&self){
    // TODO
  }
  pub fn turn_on(&mut self) {
    crate::log_enter!("sparkling.turn_on", self.name);
    if DRY_RUN {
      print_dry_run(format!("LIGHT [{}] turned ON", self.name).as_str());
      return;
    }
    self
      .serial_device
      .send_message("on")
      .expect("failed to send on message on");
    crate::log_exit!("sparkling.turn_on", self.name);
  }
  pub fn turn_off(&mut self) {
    crate::log_enter!("sparkling.turn_off", self.name);
    if DRY_RUN {
      print_dry_run(format!("LIGHT [{}] turned OFF", self.name).as_str());
      return;
    }
    self
      .serial_device
      .send_message("off")
      .expect("failed to send on message off");
    crate::log_exit!("sparkling.turn_off", self.name);
  }
  fn print(&self) {
    println!("{} {}", self.id, self.name);
  }
}
