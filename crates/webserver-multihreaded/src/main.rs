use std::net::TcpListener;
use std::thread;

use handler_sync::handle_connection;

fn main() {
  let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

  loop {
    let (connection, _) = listener.accept().unwrap();
    thread::spawn(|| {
      if let Err(e) = handle_connection(connection) {
        println!("failed to handle connection: {e}");
      }
    });
  }
}
