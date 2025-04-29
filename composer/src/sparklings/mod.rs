use crate::utils::{print_dry_run, sleep};

use crate::DRY_RUN;

const SPARKLING_ID: [u8; 3] = [0, 1, 2];

pub fn init() {
  const INIT_SPARKLING_SLEEP_SECONDS: u64 = 5000;

  all_turn_on();
  sleep(INIT_SPARKLING_SLEEP_SECONDS);
  all_turn_off();
  sleep(INIT_SPARKLING_SLEEP_SECONDS);
}

fn all_turn_on() {
  crate::log_enter!("sparkling.all_turn_on", "");
  for id in SPARKLING_ID.iter() {
    turn_on(id);
  }
  crate::log_exit!("sparkling.all_turn_on", "");
}

fn all_turn_off() {
  crate::log_enter!("sparkling.all_turn_off", "");
  for id in SPARKLING_ID.iter() {
    turn_off(id);
  }
  crate::log_exit!("sparkling.all_turn_off", "");
}

fn turn_on(id: &u8) {
  crate::log_enter!("sparkling.turn_on", id);
  if DRY_RUN {
    print_dry_run(format!("SPARKLING [{}] turned ON", id).as_str());
    return;
  }
  // TODO
  crate::log_exit!("sparkling.turn_on", id);
}

fn turn_off(id: &u8) {
  crate::log_enter!("sparkling.turn_off", id);
  if DRY_RUN {
    print_dry_run(format!("SPARKLING [{}] turned OFF", id).as_str());
    return;
  }
  // TODO
  crate::log_exit!("sparkling.turn_off", id);
}
