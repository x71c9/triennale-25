use crate::config::{self, ConfigParam};
use crate::utils::{self, print_dry_run};

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
    utils::sleep(5000, "SparklingManager new").await;
    sparkiling_manager.all_turn_off().await;
    return sparkiling_manager;
  }
  pub async fn run_sparkling(&self) {
    self.sparkling_a.run_sparkling().await;
    utils::sleep(1000, "SparklingManager run_sparkling").await;
    self.sparkling_b.run_sparkling().await;
    utils::sleep(1000, "SparklingManager run_sparkling").await;
    self.sparkling_c.run_sparkling().await;
  }
  pub async fn all_turn_on(&mut self) {
    crate::log_enter!("sparkling.all_turn_on", "");
    self.sparkling_a.turn_on().await;
    utils::sleep(2000, "SparklingManager all_turn_on").await;
    self.sparkling_b.turn_on().await;
    utils::sleep(2000, "SparklingManager all_turn_on").await;
    self.sparkling_c.turn_on().await;
    crate::log_exit!("sparkling.all_turn_on", "");
  }
  pub async fn all_turn_off(&mut self) {
    crate::log_enter!("sparkling.all_turn_off", "");
    self.sparkling_a.turn_off().await;
    utils::sleep(2000, "SparklingManager all_turn_off").await;
    self.sparkling_b.turn_off().await;
    utils::sleep(2000, "SparklingManager all_turn_off").await;
    self.sparkling_c.turn_off().await;
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
    self.turn_on().await;
    utils::sleep(1000 * 10, "Sparkling run_sparkling").await;
    self.turn_off().await;
  }
  pub async fn turn_on(&self) {
    crate::log_enter!("sparkling.turn_on", self.name);
    if config::get(ConfigParam::DRYRUN) {
      print_dry_run(format!("SPARKLING [{}] turned ON", self.name).as_str());
      crate::log_exit!("sparkling.turn_on", self.name);
      return;
    }
    invoke_service(self.service_name, "on").await;
    crate::log_exit!("sparkling.turn_on", self.name);
  }
  pub async fn turn_off(&self) {
    crate::log_enter!("sparkling.turn_off", self.name);
    if config::get(ConfigParam::DRYRUN) {
      print_dry_run(format!("SPARKLING [{}] turned OFF", self.name).as_str());
      crate::log_exit!("sparkling.turn_off", self.name);
      return;
    }
    invoke_service(self.service_name, "off").await;
    crate::log_exit!("sparkling.turn_off", self.name);
  }
  fn print(&self) {
    println!("SPARKLING {} {}", self.id, self.name);
  }
}

async fn invoke_service(path: &str, state: &str) {
  let url = format!("http://{}/{path}?state={state}", SPARKLING_SERVICE_IP);
  let response = reqwest::get(&url)
    .await
    .expect("SPARKLING SERVICE INVOCATION FAILED");

  if !response.status().is_success() {
    panic!("Request failed with status: {}", response.status());
  }

  let body = response.text().await.expect("Failed to read response body");
  println!("Sparkling service returned: {}", body);
}

pub fn create(id: &str) -> Sparkling {
  match id {
    "1" => Sparkling::new(0, "A", "s0"),
    "2" => Sparkling::new(1, "B", "s1"),
    "3" => Sparkling::new(2, "C", "s2"),
    _ => {
      panic!("Invalid Light ID. Possible value [1-3]");
    }
  }
}
