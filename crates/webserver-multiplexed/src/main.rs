use std::collections::HashMap;
use std::io;
use std::net::TcpListener;
use std::os::fd::AsRawFd;

use epoll::{ControlOptions::*, Event, Events};
use handler_non_blocking::connection::NonBlocking;
use handler_non_blocking::handler::handler;

fn main() {
  let epoll = epoll::create(false).unwrap();

  let listener = TcpListener::bind("0.0.0.0:3000").unwrap();
  listener.set_nonblocking(true).unwrap();

  let event = Event::new(Events::EPOLLIN, listener.as_raw_fd() as _);
  epoll::ctl(epoll, EPOLL_CTL_ADD, listener.as_raw_fd(), event).unwrap();

  let mut connections = HashMap::new();

  loop {
    let mut events = [Event::new(Events::empty(), 0); 1024];
    let timeout = -1; // block forever, until something happens
    let num_events = epoll::wait(epoll, timeout, &mut events).unwrap();

    let mut completed = Vec::new();

    for event in &events[..num_events] {
      let fd = event.data as i32;

      if fd == listener.as_raw_fd() {
        match listener.accept() {
          Ok((connection, _)) => {
            connection.set_nonblocking(true).unwrap();
            let fd = connection.as_raw_fd();

            let event = Event::new(Events::EPOLLIN | Events::EPOLLOUT, fd as _);
            epoll::ctl(epoll, EPOLL_CTL_ADD, fd, event).unwrap();

            connections.insert(fd, NonBlocking::new(connection));
          }
          Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
          Err(e) => panic!("{e}"),
        }

        continue;
      }

      let connection = connections.get_mut(&fd).unwrap();

      match connection.handle(handler) {
        Ok(_) => {
          if connection.is_done() {
            completed.push(fd);
          }
        }
        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
          continue;
        }
        Err(e) => {
          panic!("{e}");
        }
      }

      for fd in &completed {
        let connection = connections.remove(fd).unwrap();
        // unregister from epoll
        connection.drop();
      }
    }
  }
}
