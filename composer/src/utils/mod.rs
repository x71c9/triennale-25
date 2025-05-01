use serialport::TTYPort;
use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;

pub enum ScriptName {
  RobotGetPosition,
  RobotSetPosition,
  RobotInit,
}

impl ScriptName {
  fn as_str(&self) -> &'static str {
    match self {
      ScriptName::RobotGetPosition => "triennale-get-position",
      ScriptName::RobotSetPosition => "triennale-set-position",
      ScriptName::RobotInit => "triennale-init",
    }
  }
}

pub struct SerialDevice {
  port: TTYPort,
}

impl SerialDevice {
  pub fn new(port_name: &str, baud_rate: u32) -> anyhow::Result<Self> {
    let port = serialport::new(port_name, baud_rate)
      .timeout(Duration::from_secs(2))
      .open_native()?;
    Ok(SerialDevice { port })
  }

  pub fn send_message(&mut self, message: &str) -> anyhow::Result<()> {
    let msg = format!("{}\n", message);
    self.port.write_all(msg.as_bytes())?;
    Ok(())
  }
}

pub fn sleep(ms: u64) {
  println!("Sleeping for {} milliseconds...", ms.to_string());
  thread::sleep(Duration::from_millis(ms));
}

pub fn get_home_dir() -> PathBuf {
  if cfg!(target_os = "windows") {
    env::var("USERPROFILE")
      .map(PathBuf::from)
      .expect("Cannot resolve Win home path")
  } else {
    env::var("HOME")
      .map(PathBuf::from)
      .expect("Cannot resolve Unix home path")
  }
}

pub fn get_scripts_dir() -> PathBuf {
  let home_dir = get_home_dir();
  let path_from_string = PathBuf::from("repos/triennale-25/composer/scripts");
  let script_dir = home_dir.join(&path_from_string);
  return script_dir;
}

pub fn invoke_script(
  script_name: &ScriptName,
  args: &[&str],
) -> Option<String> {
  let script_file_name = format!("{}.py", script_name.as_str());
  let scripts_dir = get_scripts_dir();
  let script_path = scripts_dir.join(script_file_name);
  let mut full_args = vec![script_path.to_str().unwrap()];
  full_args.extend_from_slice(args);
  let output = Command::new("python3")
    .args(&full_args)
    .output()
    .ok()
    .unwrap();

  if !output.status.success() {
    eprintln!(
      "Python script failed with exit code: {:?}",
      output.status.code()
    );
    eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
    panic!("Script failed")
  }

  return Some(String::from_utf8_lossy(&output.stdout).to_string());
}

pub fn print_dry_run(msg: &str) {
  println!("[DRY RUN]: {}", msg);
}

#[macro_export]
macro_rules! log_enter {
    ($name:expr, $( $val:expr ),* $(,)?) => {{
        let args = vec![
            $( format!("{}", $val) ),*
        ];
        if crate::DEBUG == true {
          println!("→ Entering {}({})", $name, args.join(", "));
        }
    }};
}

#[macro_export]
macro_rules! log_exit {
    ($name:expr, $( $val:expr ),* $(,)?) => {{
        let args = vec![
            $( format!("{}", $val) ),*
        ];
        if crate::DEBUG == true {
          println!("← Exiting {}({})", $name, args.join(", "));
        }
    }};
}
