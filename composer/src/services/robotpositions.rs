use serde::Serialize;
use std::{net::UdpSocket, process::Command, thread, time::Duration};

const LOCAL_BIND_ADDR: &str = "0.0.0.0:0";
const TARGET_ADDR: &str = "127.0.0.1:5000";
const PYTHON_SCRIPT_PATH: &str = "scripts/get-position.py";

pub fn send() -> std::io::Result<()> {
  let socket = UdpSocket::bind(LOCAL_BIND_ADDR)?;

  loop {
    let Some(data) = RobotPositions::from_script() else {
      eprintln!("Failed to get robot positions from Python script. Exiting.");
      return Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to get robot positions from Python script",
      ));
    };

    let json = serde_json::to_string(&data).unwrap();
    socket.send_to(json.as_bytes(), TARGET_ADDR)?;

    // println!("Sent: {}", json);

    thread::sleep(Duration::from_millis(400));
  }
}

#[derive(Serialize)]
struct RobotPositions {
  r0: f32,
  r1: f32,
  r2: f32,
  r3: f32,
}

impl RobotPositions {
  fn from_script() -> Option<Self> {
    let output = Command::new("python3") // try "python3" instead of "python"
      .arg(PYTHON_SCRIPT_PATH)
      .output()
      .ok()?;

    if !output.status.success() {
      eprintln!(
        "Python script failed with exit code: {:?}",
        output.status.code()
      );
      eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
      return None;
    }

    let raw = String::from_utf8_lossy(&output.stdout);

    let parts: Vec<&str> = raw.trim().split(';').collect();
    if parts.len() != 4 {
      eprintln!("Unexpected output format: {:?}", parts);
      return None;
    }

    let values: Result<Vec<f32>, _> = parts.iter().map(|s| s.parse()).collect();
    match values {
      Ok(vals) => Some(RobotPositions {
        r0: vals[0],
        r1: vals[1],
        r2: vals[2],
        r3: vals[3],
      }),
      Err(e) => {
        eprintln!("Error parsing values: {:?}", e);
        None
      }
    }
  }
}
