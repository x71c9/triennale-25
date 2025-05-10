use rosc::decoder::decode_udp;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
  // Bind to port 5000 on all interfaces
  let socket = UdpSocket::bind("0.0.0.0:6666")?;

  // Allow receiving broadcast messages
  socket.set_broadcast(true)?;

  println!("Listening for OSC messages on port 6666...");

  let mut buf = [0u8; 1024];

  loop {
    let (size, addr) = socket.recv_from(&mut buf)?;
    match decode_udp(&buf[..size]) {
      Ok((_, packet)) => match packet {
        rosc::OscPacket::Message(msg) => {
          if msg.addr == "/robots" {
            println!("Received from {}: {:?}", addr, msg);
          }
        }
        rosc::OscPacket::Bundle(bundle) => {
          for p in bundle.content {
            if let rosc::OscPacket::Message(msg) = p {
              if msg.addr == "/robots" {
                println!("Received from {}: {:?}", addr, msg);
              }
            }
          }
        }
      },
      Err(e) => eprintln!("Failed to decode OSC packet: {}", e),
    }
  }
}
