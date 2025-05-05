use crate::utils::{
  self, print_dry_run, MockSerialDevice, RealSerialDevice, SerialDevice,
};
use crate::DRY_RUN;

const LIGHT_SERIAL_PORT_NAME: &'static str = "/dev/tty.usbmodem14101";
const LIGHT_SERIAL_BAUD: u32 = 9600;

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
      light_a: Light::new(0, "A"),
      light_b: Light::new(1, "B"),
      light_c: Light::new(2, "C"),
      light_d: Light::new(3, "D"),
      light_e: Light::new(4, "E"),
      light_f: Light::new(5, "F"),
    };
    light_manager.all_turn_on().await;
    utils::sleep(5000).await;
    light_manager.all_turn_off().await;
    return light_manager;
  }
  pub async fn regulate_light(&self) {
    // TODO remove
    if crate::NARROW == true {
      return;
    }
    // TODO
    self.light_a.regulate_light();
  }
  pub async fn all_turn_on(&mut self) {
    // TODO remove
    if crate::NARROW == true {
      return;
    }
    crate::log_enter!("lights.all_turn_on", "");
    self.light_a.turn_on();
    utils::sleep(1000 * 2).await;
    self.light_b.turn_on();
    utils::sleep(1000 * 2).await;
    self.light_c.turn_on();
    utils::sleep(1000 * 2).await;
    self.light_d.turn_on();
    utils::sleep(1000 * 2).await;
    self.light_e.turn_on();
    utils::sleep(1000 * 2).await;
    self.light_f.turn_on();
    crate::log_exit!("lights.all_turn_on", "");
  }
  pub async fn all_turn_off(&mut self) {
    // TODO remove
    if crate::NARROW == true {
      return;
    }
    crate::log_enter!("lights.all_turn_off", "");
    self.light_a.turn_off();
    utils::sleep(1000 * 2).await;
    self.light_b.turn_off();
    utils::sleep(1000 * 2).await;
    self.light_c.turn_off();
    utils::sleep(1000 * 2).await;
    self.light_d.turn_off();
    utils::sleep(1000 * 2).await;
    self.light_e.turn_off();
    utils::sleep(1000 * 2).await;
    self.light_f.turn_off();
    crate::log_exit!("lights.all_turn_off", "");
  }
}

pub struct Light {
  pub id: u8,
  pub name: &'static str,
  pub serial_device: Box<dyn SerialDevice>,
}

impl Light {
  pub fn new(id: u8, name: &'static str) -> Self {
    let serial_device: Box<dyn SerialDevice> = if DRY_RUN {
      Box::new(
        MockSerialDevice::new(LIGHT_SERIAL_PORT_NAME, LIGHT_SERIAL_BAUD)
          .expect("Cannot initialize MockSerialDevice"),
      )
    } else {
      Box::new(
        RealSerialDevice::new(LIGHT_SERIAL_PORT_NAME, LIGHT_SERIAL_BAUD)
          .expect("Cannot initialize RealSerialDevice"),
      )
    };
    let light: Light = Light {
      id,
      name,
      serial_device,
    };
    light.print();
    return light;
  }
  pub fn regulate_light(&self) {
    // TODO
  }
  pub fn turn_on(&mut self) {
    // TODO remove
    if crate::NARROW == true {
      return;
    }
    crate::log_enter!("lights.turn_on", self.name);
    if DRY_RUN {
      print_dry_run(format!("LIGHT [{}] turned ON", self.name).as_str());
      crate::log_exit!("lights.turn_on", self.name);
      return;
    }
    self
      .serial_device
      .send_message("on")
      .expect("failed to send on message on");
    crate::log_exit!("lights.turn_on", self.name);
  }
  pub fn turn_off(&mut self) {
    // TODO remove
    if crate::NARROW == true {
      return;
    }
    crate::log_enter!("lights.turn_off", self.name);
    if DRY_RUN {
      print_dry_run(format!("LIGHT [{}] turned OFF", self.name).as_str());
      crate::log_exit!("lights.turn_off", self.name);
      return;
    }
    self
      .serial_device
      .send_message("off")
      .expect("failed to send on message off");
    crate::log_exit!("lights.turn_off", self.name);
  }
  fn print(&self) {
    println!("LIGHT {} {}", self.id, self.name);
  }
}
