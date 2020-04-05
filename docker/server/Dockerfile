FROM rust:latest as server-builder

RUN apt-get update

RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/server

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/server*

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch

# RUN addgroup -g 1000 server

# RUN adduser -D -s /bin/sh -u 1000 -G server server

# WORKDIR /home/server/bin/

COPY --from=server-builder /usr/src/server/target/x86_64-unknown-linux-musl/release/server .

# RUN chown server:server server

# USER server

# COPY ./.env .

CMD ["./server"]
