FROM rust:latest as builder

WORKDIR /usr/src/myapp
COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release --bins


FROM scratch

COPY --from=builder /usr/src/myapp/target/x86_64-unknown-linux-musl/release /usr/local/bin

EXPOSE 3000

