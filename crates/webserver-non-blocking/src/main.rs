use std::io;
use std::net::TcpListener;

use handler_non_blocking::connection::NonBlocking;
use handler_non_blocking::handler::handler;

fn main() {
  let listener = TcpListener::bind("0.0.0.0:3000").unwrap();
  listener.set_nonblocking(true).unwrap();

  let mut connections = Vec::new();

  loop {
    match listener.accept() {
      Ok((connection, _)) => {
        connection.set_nonblocking(true).unwrap();

        connections.push(NonBlocking::new(connection));
      }
      Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
      Err(e) => panic!("{e}"),
    };

    let mut completed = Vec::new();

    for (i, conn) in connections.iter_mut().enumerate() {
      match conn.handle(|s| handler(s)) {
        Ok(_) => {
          if conn.is_done() {
            completed.push(i);
          }
        }
        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
          continue;
        }
        Err(e) => {
          panic!("{e}");
        }
      }
    }

    for i in completed.into_iter().rev() {
      connections.remove(i);
    }
  }
}
