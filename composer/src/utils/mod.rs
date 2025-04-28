use std::env;
use std::path::PathBuf;
use std::process::Command;

pub enum ScriptName {
  GetPosition,
  SetPosition,
}

impl ScriptName {
  fn as_str(&self) -> &'static str {
    match self {
      ScriptName::GetPosition => "get-position",
      ScriptName::SetPosition => "set-position",
    }
  }
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
  let path_from_string = PathBuf::from("repos/triennale-25/scripts");
  let script_dir = home_dir.join(&path_from_string);
  return script_dir;
}

pub fn invoke_script(script_name: &ScriptName, args: &[&str]) {
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
}

pub fn invoke_set_position(id: &u8, position: &f64) {
  invoke_script(
    &ScriptName::SetPosition,
    &[id.to_string().as_str(), position.to_string().as_str()],
  );
}

pub fn invoke_get_position(id: &u8) {
  invoke_script(
    &ScriptName::GetPosition,
    &[id.to_string().as_str()],
  );
}
