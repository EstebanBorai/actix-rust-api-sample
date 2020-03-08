# rust-actix-postgres
:crab: Actix based REST Web API to learn Rust

## Running Locally

- Docker

### Docker
This project makes use of a Postgres database and a Redis server which run on Docker.
Build and set "up" the docker containers using `docker-compose` as follows:

```sh
docker-compose -f docker-compose.yml up --build
```
### Dependencies

Depedencies are defined in the *Cargo.toml* file, these dependencies are automatically installed when running euther `cargo run` or `cargo build` and will be stored in the `~/.cargo/` directory.

Diesel CLI (A Rust binary) is required in order to build schemas and run migrations for the models.

Install Diesel CLI using the following command:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

<em>
	For example, if you only have PostgreSQL installed, you can use this to install diesel_cli with only PostgreSQL
</em>
<br />
- diesel.rs, Getting Started Guide

Then run the following command to create a migration directory:

```bash
diesel setup
```

> The database should be active for the moment diesel cli is beign used.

Finally run migrations:

```bash
diesel migration run
```
