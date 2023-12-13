use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use self::status::ConnectionStatus;

mod status;

pub struct NonBlocking {
  conn: TcpStream,
  status: ConnectionStatus,
}

impl NonBlocking {
  pub fn new(conn: TcpStream) -> Self {
    NonBlocking {
      conn: conn,
      status: ConnectionStatus::new_read(),
    }
  }

  pub fn is_done(&self) -> bool {
    self.status == ConnectionStatus::Done
  }

  pub fn drop(self) {
    drop(self.conn);
  }

  pub fn handle<F>(&mut self, handler: F) -> io::Result<()>
  where
    F: FnOnce(String) -> String,
  {
    if let ConnectionStatus::Read { .. } = self.status {
      let request = self.read()?;
      let response = handler(String::from(request));
      self.status = ConnectionStatus::new_write(response);
    }
    if let ConnectionStatus::Write { .. } = self.status {
      self.write()?;
      self.status = ConnectionStatus::Flush;
    }
    if let ConnectionStatus::Flush = self.status {
      self.flush()?;
      self.status = ConnectionStatus::Done;
    }
    Ok(())
  }

  fn read(&mut self) -> io::Result<String> {
    if let ConnectionStatus::Read { buf, pos } = &mut self.status {
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
    if let ConnectionStatus::Write { buf, pos } = &mut self.status {
      let n = self.conn.write(&buf.as_bytes()[*pos..])?;
      if n == 0 { 
        println!("client disconnected unexpectedly");
        self.status = ConnectionStatus::Done;
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
