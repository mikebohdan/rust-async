use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle_connection(mut connection: TcpStream) -> io::Result<()> {
  let mut read = 0;
  let mut request = [0u8; 1024];

  loop {
    let num_bytes = connection.read(&mut request[read..])?;
    read += num_bytes;

    // check for an end of income data
    if request.get(read - 4..read) == Some(b"\r\n\r\n") { 
      break;
    }
  }

  let request = String::from_utf8_lossy(&request[..read]);
  println!("{request}");

  let response = concat!(
    "HTTP/1.1 200 OK\r\n",
    "Connection-Length: 22\n",
    "Connection: close\r\n\r\n",
    "<h1>Hello, World!</h1>",
  );

  let mut written = 0;

  loop {
    let num_bytes = connection.write(response[written..].as_bytes())?;

    if num_bytes == 0 {
      println!("client disconnected unexpectedly");
      return Ok(())
    }

    written += num_bytes;

    if written == response.len() {
      break;
    }
  }

  connection.flush()
}
