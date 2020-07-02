<div align="center">
  <img src="https://www.rust-lang.org/static/images/rust-logo-blk.svg" height="144" width="144" />
  <h1>rust-webapi</h1>
  <small>🦀 A Web API written in Rust for learning purposes </small>
</div>

## Motivation
The main goal of this project is to learn about the *Rust* programming language
following a set of complete and rich in knowledge guides from Tore Pettersen in his
blog "[cloudmaker.dev](https://cloudmaker.dev/)".

## Requirements
- [Rust](https://rustup.rs/)
- [Docker](https://www.docker.com/)

## Getting Started
Some setup must be done before getting to run the application.
The following will walk you through the application environment setup.

When the setup is ready, run the application issuing the following command
from the project directory:

```sh
cargo run

# expected output
➜  rust-webapi git:(master) cargo run
   Compiling ....
   Compiling server v0.1.0 (~/rust-webapi)
    Finished dev [unoptimized + debuginfo] target(s) in 47.72s
     Running `target/debug/server`
[2020-07-02T17:38:43Z WARN  server] ⚠️   Running in development mode
[2020-07-02T17:38:43Z INFO  server::db] Initializing DB Connection
[2020-07-02T17:38:43Z INFO  server] 🚀  Server ready at http://127.0.0.1:7878
[2020-07-02T17:38:43Z INFO  actix_server::builder] Starting 4 workers
[2020-07-02T17:38:43Z INFO  actix_server::builder] Starting "actix-web-service-127.0.0.1:7878" service on 127.0.0.1:7878
[2020-07-02T17:38:44Z DEBUG trust_dns_resolver::async_resolver::background] trust-dns resolver running
[2020-07-02T17:38:44Z INFO  actix_redis::redis] Connected to redis server: 127.0.0.1:6379
[2020-07-02T17:38:44Z INFO  actix_redis::redis] Connected to redis server: 127.0.0.1:6379
[2020-07-02T17:38:44Z INFO  actix_redis::redis] Connected to redis server: 127.0.0.1:6379
[2020-07-02T17:38:44Z INFO  actix_redis::redis] Connected to redis server: 127.0.0.1:6379
```

### Running the Postgres Database, Redis and PgAdmin with docker
A `docker-compose.yml` file is available in the project directory
with the definition of the following services:

Service | Image | Description 
--- | --- | ---
`database`| `postgres:9.6` | Postgres Database
`redis` | `redis:rc-buster` | Redis
`pgadmin` | `dpage/pgadmin4:latest` | A *Postgres* database admin

A one line command is available to run as a *shell/bash* file in named `run-resources.sh`.

When running `run-resources.sh` the services in the table above will be the
only services running. The *Rust* server must be started manually
using `cargo run` from the project directory.

> The complete environment is available to run also. Issue `docker-compose up --build` from the project directory to build and run the complete set of services including the *Rust* server.

### Setup Diesel
Using `cargo install` command, install `diesel_cli` binary:

```sh
cargo install diesel_cli --no-default-features --features postgres
```

If theres no `diesel.toml` file in the project directory,
run `diesel setup` to generate such file.

Expected output:

```log
Creating migrations/2020-04-21-005225_create_user/up.sql
Creating migrations/2020-04-21-005225_create_user/down.sql
```

> To generate migrations, run `diesel migration generate <migration name>`

> In order to run migrations and update database tables with the last schema run `diesel migration run`

### Running on Ubuntu
For Ubuntu is required to install the following package:

```bash
apt-get install libpq-dev
```

Required to build: `migrations_macros v1.4.2`
[Source](https://github.com/filecoin-project/replication-game/issues/45)