# Learning Async Rust
[Learning Async Rust With Entirely Too Many Web Servers](https://ibraheem.ca/posts/too-many-web-servers/)

## Running
For non-Linux users as myself added docker file.

To build the project run:
```bash
docker build . --tag rust-async
```

To run the project run:
```bash
docker run -d -p 3000:3000 rust-async <executable-name>
```

## A Simple Web Server
- [Guide](https://ibraheem.ca/posts/too-many-web-servers/#a-simple-web-server)
- [Crate](./crates/webserver-simple/)
- **Binary Name** `webserver-simple`

## A Multithreaded Server
- [Guide](https://ibraheem.ca/posts/too-many-web-servers/#a-multithreaded-server)
- [Crate](./crates/webserver-multihreaded/)
- **Binary Name** `webserver-multihreaded`

## A Non-Blocking Server
- [Guide](https://ibraheem.ca/posts/too-many-web-servers/#a-non-blocking-server)
- [Crate](./crates/webserver-non-blocking/)
- **Binary Name** `webserver-non-blocking`

## A Multiplexed Server
- [Guide](https://ibraheem.ca/posts/too-many-web-servers/#a-multiplexed-server)
- [Crate](./crates/webserver-multiplexed/)
- **Binary Name** `webserver-multiplexed`
