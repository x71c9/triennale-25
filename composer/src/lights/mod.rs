use crate::utils::{print_dry_run, sleep, SerialRelay};

use crate::DRY_RUN;

const LIGHTS_ID: [u8; 6] = [0, 1, 2, 3, 4, 5];

pub fn init() {
  let port_name = "/dev/tty.usbmodem14101";
  let baud = 9600;

  let mut relay = SerialRelay::new(port_name, baud).unwrap();
  relay.send_message("ON").expect("Failed to send ON message");
  std::thread::sleep(std::time::Duration::from_secs(2));
  relay
    .send_message("OFF")
    .expect("Failed to send OFF message");

  const INIT_LIGHTS_SLEEP_SECONDS: u64 = 5000;

  all_turn_on();
  sleep(INIT_LIGHTS_SLEEP_SECONDS);
  all_turn_off();
  sleep(INIT_LIGHTS_SLEEP_SECONDS);
}

fn all_turn_on() {
  crate::log_enter!("lights.all_turn_on", "");
  for id in LIGHTS_ID.iter() {
    turn_on(id);
  }
  crate::log_exit!("lights.all_turn_on", "");
}

fn all_turn_off() {
  crate::log_enter!("lights.all_turn_off", "");
  for id in LIGHTS_ID.iter() {
    turn_off(id);
  }
  crate::log_exit!("lights.all_turn_off", "");
}

fn turn_on(id: &u8) {
  crate::log_enter!("lights.turn_on", id);
  if DRY_RUN {
    print_dry_run(format!("LIGHT [{}] turned ON", id).as_str());
    return;
  }
  // TODO
  crate::log_exit!("lights.turn_on", id);
}

fn turn_off(id: &u8) {
  crate::log_enter!("lights.turn_off", id);
  if DRY_RUN {
    print_dry_run(format!("LIGHT [{}] turned OFF", id).as_str());
    return;
  }
  // TODO
  crate::log_exit!("lights.turn_off", id);
}
