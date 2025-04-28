mod services;
mod robots;
mod utils;

use services::robotpositions::send;
use robots::init;
use std::thread;

fn main() {

  let handle = thread::spawn(|| {
    if let Err(e) = send() {
        eprintln!("Error in send: {}", e);
    }
  });

  println!("Main thread is continuing...");

  init();

  handle.join().unwrap();
}
