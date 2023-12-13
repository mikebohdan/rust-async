pub fn handler(request: String) -> String {
  println!("{request}");
  String::from(concat!(
    "HTTP/1.1 200 OK\r\n",
    "Content-Length: 12\n",
    "Connection: close\r\n\r\n",
    "Hello world!"
  ))
}