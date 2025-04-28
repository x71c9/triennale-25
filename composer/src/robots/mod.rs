use crate::utils::{invoke_get_position, invoke_set_position, sleep};

const ROBOT_IDS: [u8; 4] = [0, 1, 2, 3];

pub fn init() {
  all_go_to(&0.0);
  get_position(&0);
  sleep(1000);
  all_go_to(&1.0);
}

fn all_go_to(position: &f64) {
  crate::log_enter!("all_go_to", position);
  for id in ROBOT_IDS.iter() {
    set_position(id, position);
  }
  crate::log_exit!("all_go_to", position);
}

fn set_position(id: &u8, position: &f64) {
  crate::log_enter!("set_position", id, position);
  invoke_set_position(id, position);
  crate::log_exit!("set_position", id, position);
}

fn get_position(id: &u8) {
  invoke_get_position(id);
}
