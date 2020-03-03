# rust-actix-rentals
:crab: Actix based REST Web API to learn Rust

## Running Locally

- Docker

### Docker
Docker is used to setup a Postgres database.
The first step is building the Dockerfile as follows:

```bash
docker build -t actix-rentals-db ./docker 
```

```bash
docker run actix-rentals-db
```
