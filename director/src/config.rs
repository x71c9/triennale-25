use std::sync::OnceLock;

#[derive(Debug)]
pub struct Config {
  pub debug: bool,
  pub dry_run: bool,
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();

pub enum ConfigParam {
  VERBOSE,
  DRYRUN,
}

pub fn get(field: ConfigParam) -> bool {
  let config = CONFIG.get().expect("Config not initialized");
  match field {
    ConfigParam::VERBOSE => config.debug,
    ConfigParam::DRYRUN => config.dry_run,
  }
}
