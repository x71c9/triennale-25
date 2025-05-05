use crate::utils::{self, print_dry_run};
use crate::DRY_RUN;
use reqwest::blocking;

const SPARKLING_SERVICE_IP: &str = "192.168.125.3";

pub struct SparklingManager {
  pub sparkling_a: Sparkling,
  pub sparkling_b: Sparkling,
  pub sparkling_c: Sparkling,
}

impl SparklingManager {
  pub async fn new() -> Self {
    let mut sparkiling_manager: SparklingManager = SparklingManager {
      sparkling_a: Sparkling::new(0, "A", "s0"),
      sparkling_b: Sparkling::new(1, "B", "s1"),
      sparkling_c: Sparkling::new(2, "C", "s2"),
    };
    sparkiling_manager.all_turn_on().await;
    utils::sleep(5000).await;
    sparkiling_manager.all_turn_off().await;
    return sparkiling_manager;
  }
  pub async fn run_sparkling(&self) {
    // TODO remove
    // if crate::NARROW == true {
    //   return;
    // }
    self.sparkling_a.run_sparkling().await;
    utils::sleep(1000).await;
    self.sparkling_b.run_sparkling().await;
    utils::sleep(1000).await;
    self.sparkling_c.run_sparkling().await;
  }
  pub async fn all_turn_on(&mut self) {
    // TODO remove
    // if crate::NARROW == true {
    //   return;
    // }
    crate::log_enter!("sparkling.all_turn_on", "");
    self.sparkling_a.turn_on();
    utils::sleep(2000).await;
    self.sparkling_b.turn_on();
    utils::sleep(2000).await;
    self.sparkling_c.turn_on();
    crate::log_exit!("sparkling.all_turn_on", "");
  }
  pub async fn all_turn_off(&mut self) {
    // TODO remove
    // if crate::NARROW == true {
    //   return;
    // }
    crate::log_enter!("sparkling.all_turn_off", "");
    self.sparkling_a.turn_off();
    utils::sleep(2000).await;
    self.sparkling_b.turn_off();
    utils::sleep(2000).await;
    self.sparkling_c.turn_off();
    crate::log_exit!("sparkling.all_turn_off", "");
  }
}

pub struct Sparkling {
  pub id: u8,
  pub name: &'static str,
  pub service_name: &'static str,
}

impl Sparkling {
  pub fn new(id: u8, name: &'static str, service_name: &'static str) -> Self {
    let light: Sparkling = Sparkling {
      id,
      name,
      service_name,
    };
    light.print();
    return light;
  }
  pub async fn run_sparkling(&self) {
    // TODO remove
    // if crate::NARROW == true {
    //   return;
    // }
    self.turn_on();
    utils::sleep(1000 * 10).await;
    self.turn_off();
  }
  pub fn turn_on(&self) {
    // TODO remove
    // if crate::NARROW == true {
    //   return;
    // }
    crate::log_enter!("sparkling.turn_on", self.name);
    if DRY_RUN {
      print_dry_run(format!("SPARKLING [{}] turned ON", self.name).as_str());
      crate::log_exit!("sparkling.turn_on", self.name);
      return;
    }
    invoke_service(self.service_name, "on");
    crate::log_exit!("sparkling.turn_on", self.name);
  }
  pub fn turn_off(&self) {
    // TODO remove
    // if crate::NARROW == true {
    //   return;
    // }
    crate::log_enter!("sparkling.turn_off", self.name);
    if DRY_RUN {
      print_dry_run(format!("SPARKLING [{}] turned OFF", self.name).as_str());
      crate::log_exit!("sparkling.turn_off", self.name);
      return;
    }
    invoke_service(self.service_name, "off");
    crate::log_exit!("sparkling.turn_off", self.name);
  }
  fn print(&self) {
    println!("SPARKLING {} {}", self.id, self.name);
  }
}

fn invoke_service(path: &str, state: &str) {
  let url = format!("http://{}/{path}?state={state}", SPARKLING_SERVICE_IP);
  let response =
    blocking::get(&url).expect("SPARKLING SERVICE INVOKACTION FAILED");
  if !response.status().is_success() {
    panic!("Request failed with status: {}", response.status())
  }
  println!(
    "Sparkling service returned: {}",
    response.text().expect("Failed to read response body")
  );
}
