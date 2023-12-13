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
Single-threaded processing of incoming requests. The base implementation is straightforward, 
but I moved handling logic to the [handler-sync](./crates/handler-sync/) crate to reuse it in 
[the multithreaded web server]((#a-multithreaded-server)).

- [Guide](https://ibraheem.ca/posts/too-many-web-servers/#a-simple-web-server)
- [Crate](./crates/webserver-simple/)
- **Binary Name** `webserver-simple`

## A Multithreaded Server
The main difference compared to the previous one is that now we are spawning a new thread on each connection. 
Since we have implemented shared logic in the [handler-sync](./crates/handler-sync/) crate, 
we can see even better that the difference is tiny.

- [Guide](https://ibraheem.ca/posts/too-many-web-servers/#a-multithreaded-server)
- [Crate](./crates/webserver-multihreaded/)
- **Binary Name** `webserver-multihreaded`

## A Non-Blocking Server
Here is starting something interesting. This implementation is single-threaded as the first one, 
but it can handle multiple requests at the same time.

The implementation from the post is pretty neat, but I didn't quite like the GOTO approach. 
On the other hand, the implementation of this and [the multiplexed server](#a-multiplexed-server) are not very different, 
so we can write [yet another shared crate](./crates/handler-non-blocking/).

> [Go To Statement Considered Harmful.](https://homepages.cwi.nl/~storm/teaching/reader/Dijkstra68.pdf)
>
> -- <cite>Edgar Dijkstra</cite>

- [Guide](https://ibraheem.ca/posts/too-many-web-servers/#a-non-blocking-server)
- [Crate](./crates/webserver-non-blocking/)
- **Binary Name** `webserver-non-blocking`

## A Multiplexed Server
We are "outsourcing" some work to the [epoll](https://www.man7.org/linux/man-pages/man7/epoll.7.html) in this approach. 
The handling logic is fully re-used from [the non-blocking server implementation](#a-non-blocking-server).

Unfortunately, the implementation of the original blog post used a GOTO statement again. Nothing changed in its harmfulness 🙃.

- [Guide](https://ibraheem.ca/posts/too-many-web-servers/#a-multiplexed-server)
- [Crate](./crates/webserver-multiplexed/)
- **Binary Name** `webserver-multiplexed`
