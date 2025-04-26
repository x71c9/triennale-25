use chrono::Local;
use serde::Deserialize;
use std::net::UdpSocket;

#[derive(Deserialize)]
struct RobotPositions {
  r0: f32,
  r1: f32,
  r2: f32,
  r3: f32,
}

fn main() -> std::io::Result<()> {
  let socket = UdpSocket::bind("0.0.0.0:5000")?;
  println!("Listening for robot positions on UDP port 5000...");

  let mut buf = [0u8; 1024];

  loop {
    let (amt, src) = socket.recv_from(&mut buf)?;

    let msg = String::from_utf8_lossy(&buf[..amt]);
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");

    match serde_json::from_str::<RobotPositions>(&msg) {
      Ok(data) => println!(
        "[{}] From {} → r0: {:.3}, r1: {:.3}, r2: {:.3}, r3: {:.3}",
        timestamp, src, data.r0, data.r1, data.r2, data.r3
      ),
      Err(_) => {
        println!("[{}] Invalid message from {}: {}", timestamp, src, msg)
      }
    }
  }
}
