use crate::utils::{invoke_get_position, invoke_set_position, sleep};

const ROBOT_IDS: [u8; 4] = [0, 1, 2, 3];

pub fn init() {
  let response = get_position(&0);
  println!("Get position response: {}", response);

  const INIT_ROBOT_SLEEP_SECONDS: u64 = 1000;

  all_go_to(&0.0);
  sleep(INIT_ROBOT_SLEEP_SECONDS);
  all_go_to(&1.0);
  sleep(INIT_ROBOT_SLEEP_SECONDS);
  all_go_to(&0.0);
  sleep(INIT_ROBOT_SLEEP_SECONDS);
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

fn get_position(id: &u8) -> String {
  let response = invoke_get_position(id);
  return response;
}
