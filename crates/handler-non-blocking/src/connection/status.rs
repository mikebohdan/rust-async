#[derive(PartialEq)]
pub enum ConnectionStatus {
  Read { buf: [u8; 1024], pos: usize },
  Write { buf: String, pos: usize },
  Flush,
  Done,
}

impl ConnectionStatus {
  pub fn new_read() -> Self {
    Self::Read {
      buf: [0; 1024],
      pos: 0,
    }
  }

  pub fn new_write(response: String) -> Self {
    Self::Write {
      buf: response,
      pos: 0,
    }
  }
}