use crate::utils::{
  self, print_dry_run, MockSerialDevice, RealSerialDevice, SerialDevice,
};
use crate::config::{self, ConfigParam};

// const LIGHT_SERIAL_PORT_NAME: &'static str = "/dev/tty.usbmodem14101";
const LIGHT_SERIAL_PORT_NAME: &'static str = "/dev/ttyACM0";
const LIGHT_SERIAL_BAUD: u32 = 115200;

pub struct LightManager {
  pub light_a: Light,
  pub light_b: Light,
  pub light_c: Light,
  pub light_d: Light,
  pub light_e: Light,
  pub light_f: Light,
}

impl LightManager {
  pub async fn new() -> Self {
    let mut light_manager: LightManager = LightManager {
      light_a: Light::new(0, "A", 5).await,
      light_b: Light::new(1, "B", 3).await,
      light_c: Light::new(2, "C", 1).await,
      light_d: Light::new(3, "D", 2).await,
      light_e: Light::new(4, "E", 4).await,
      light_f: Light::new(5, "F", 6).await,
    };
    light_manager.all_turn_on().await;
    utils::sleep(5000, "LightManager new").await;
    light_manager.all_turn_off().await;
    return light_manager;
  }
  pub async fn regulate_light(&mut self) {
    self.light_a.dim(5000);
  }
  pub async fn all_turn_on(&mut self) {
    crate::log_enter!("lights.all_turn_on", "");
    self.light_a.turn_on();
    utils::sleep(1000 * 2, "LightManager all_turn_on").await;
    self.light_b.turn_on();
    utils::sleep(1000 * 2, "LightManager all_turn_on").await;
    self.light_c.turn_on();
    utils::sleep(1000 * 2, "LightManager all_turn_on").await;
    self.light_d.turn_on();
    utils::sleep(1000 * 2, "LightManager all_turn_on").await;
    self.light_e.turn_on();
    utils::sleep(1000 * 2, "LightManager all_turn_on").await;
    self.light_f.turn_on();
    crate::log_exit!("lights.all_turn_on", "");
  }
  pub async fn all_turn_off(&mut self) {
    crate::log_enter!("lights.all_turn_off", "");
    self.light_a.turn_off();
    utils::sleep(1000 * 2, "LightManager all_turn_off").await;
    self.light_b.turn_off();
    utils::sleep(1000 * 2, "LightManager all_turn_off").await;
    self.light_c.turn_off();
    utils::sleep(1000 * 2, "LightManager all_turn_off").await;
    self.light_d.turn_off();
    utils::sleep(1000 * 2, "LightManager all_turn_off").await;
    self.light_e.turn_off();
    utils::sleep(1000 * 2, "LightManager all_turn_off").await;
    self.light_f.turn_off();
    crate::log_exit!("lights.all_turn_off", "");
  }
}

pub struct Light {
  pub id: u8,
  pub name: &'static str,
  pub serial_device: Box<dyn SerialDevice>,
  pub serial_channel: u8,
}

impl Light {
  pub async fn new(id: u8, name: &'static str, serial_channel: u8) -> Self {
    let serial_device: Box<dyn SerialDevice> = if config::get(ConfigParam::DRYRUN) {
      Box::new(
        MockSerialDevice::new(LIGHT_SERIAL_PORT_NAME, LIGHT_SERIAL_BAUD)
          .expect("Cannot initialize MockSerialDevice"),
      )
    } else {
      Box::new(
        RealSerialDevice::new(LIGHT_SERIAL_PORT_NAME, LIGHT_SERIAL_BAUD).await
          .expect("Cannot initialize RealSerialDevice"),
      )
    };
    let light: Light = Light {
      id,
      name,
      serial_device,
      serial_channel,
    };
    light.print();
    return light;
  }
  pub fn dim(&mut self, value: u16) {
    crate::log_enter!("lights.dim", self.name);
    if config::get(ConfigParam::DRYRUN) {
      print_dry_run(format!("LIGHT [{}] dimmed {}", self.name, value).as_str());
      crate::log_exit!("lights.turn_off", self.name);
      return;
    }
    let message = format!("DIM {} {}", self.serial_channel.to_string(), value.to_string());
    self
      .serial_device
      .send_message(message.as_str())
      .expect("failed to send on message dim");
    crate::log_exit!("lights.dim", self.name);
  }
  pub fn turn_on(&mut self) {
    crate::log_enter!("lights.turn_on", self.name);
    if config::get(ConfigParam::DRYRUN) {
      print_dry_run(format!("LIGHT [{}] turned ON", self.name).as_str());
      crate::log_exit!("lights.turn_on", self.name);
      return;
    }
    let message = format!("DIM {} 10000", self.serial_channel.to_string());
    self
      .serial_device
      .send_message(message.as_str())
      .expect("failed to send on message on");
    crate::log_exit!("lights.turn_on", self.name);
  }
  pub fn turn_off(&mut self) {
    crate::log_enter!("lights.turn_off", self.name);
    if config::get(ConfigParam::DRYRUN) {
      print_dry_run(format!("LIGHT [{}] turned OFF", self.name).as_str());
      crate::log_exit!("lights.turn_off", self.name);
      return;
    }
    let message = format!("DIM {} 0", self.serial_channel.to_string());
    self
      .serial_device
      .send_message(message.as_str())
      .expect("failed to send on message off");
    crate::log_exit!("lights.turn_off", self.name);
  }
  fn print(&self) {
    println!("LIGHT {} {}", self.id, self.name);
  }
}

pub async fn create(id: &str) -> Light {
  match id {
    "1" => Light::new(0, "A", 5).await,
    "2" => Light::new(1, "B", 3).await,
    "3" => Light::new(2, "C", 1).await,
    "4" => Light::new(3, "D", 2).await,
    "5" => Light::new(4, "E", 4).await,
    "6" => Light::new(5, "F", 6).await,
    _ => {
      panic!("Invalid Light ID. Possible value [1-6]");
    }
  }
}
