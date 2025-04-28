use crate::utils::{invoke_set_position, invoke_get_position};

const ROBOT_IDS: [u8; 4] = [0, 1, 2, 3];

pub fn init() {
  all_go_to(&0.0);
  get_position(&0);
}

fn all_go_to(position: &f64) {
  for id in ROBOT_IDS.iter() {
    set_position(id, position);
  }
}

fn set_position(id: &u8, position: &f64) {
  invoke_set_position(id, position);
}

fn get_position(id: &u8) {
  invoke_get_position(id);
}
