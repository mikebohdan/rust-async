use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(PartialEq)]
enum Status {
  Read { buf: [u8; 1024], pos: usize },
  Write { buf: String, pos: usize },
  Flush,
  Done,
}

impl Status {
  fn new_read() -> Self {
    Self::Read {
      buf: [0; 1024],
      pos: 0,
    }
  }
}

pub struct NonBlocking {
  conn: TcpStream,
  status: Status,
}

impl NonBlocking {
  pub fn new(conn: TcpStream) -> Self {
    NonBlocking {
      conn: conn,
      status: Status::new_read(),
    }
  }

  pub fn is_done(&self) -> bool {
    self.status == Status::Done
  }

  pub fn handle<F>(&mut self, handler: F) -> io::Result<()>
  where
    F: FnOnce(String) -> String,
  {
    if let Status::Read { .. } = self.status {
      let request = self.read()?;
      let response = handler(String::from(request));
      self.status = Status::Write {
        buf: response,
        pos: 0,
      };
    }
    if let Status::Write { .. } = self.status {
      self.write()?;
      self.status = Status::Flush;
    }
    if let Status::Flush = self.status {
      self.flush()?;
      self.status = Status::Done;
    }
    Ok(())
  }

  fn read(&mut self) -> io::Result<String> {
    if let Status::Read { buf, pos } = &mut self.status {
      loop {
        *pos += self.conn.read(&mut buf[*pos..])?;

        if buf.get(*pos - 4..*pos) == Some(b"\r\n\r\n") {
          break;
        }

        println!("{}", String::from_utf8_lossy(buf));
      }

      return Ok(String::from_utf8_lossy(&buf[..*pos]).to_string());
    }
    panic!();
  }

  fn write(&mut self) -> io::Result<()> {
    if let Status::Write { buf, pos } = &mut self.status {
      let n = self.conn.write(&buf.as_bytes()[*pos..])?;
      if n == 0 { 
        println!("client disconnected unexpectedly");
        self.status = Status::Done;
      } else {
        *pos += n;
      }
      return Ok(())
    }

    panic!();
  }

  fn flush(&mut self) -> io::Result<()> {
    self.conn.flush()
  }
}
