# ChocoAPI

API/Backend for projects of Kokoa club

## Requirements

- rustup
- lld
- clang
- docker && docker compose
- cargo install sqlx-cli
- postgres running (use `docker compose up -d postgres`)

## Environment setup

Currently app configuration is done through the configuration files in the
`/configuration` directory and can be overwritten locally with an `.env` file.
Copy the `.env.sample` file removing the `.sample` extension and modify as needed.

Run `cargo sqlx prepare -- --bin chocoapi` each time a database query is added,
modified or deleted.

## Running

Postgres must be listening before running `cargo run`. For example

```sh
docker compose up -d postgres
cargo run
```

The application can also be run using docker compose, this emulates
a production environment.

```sh
docker compose up
```

That will spawn a docker compose instance running the api container with
postgres and redis as services. If the api container is not already built, that
command will also build it the first time, but to rebuild it after changing the
code you'll need to use the `--build` flag:

```sh
docker compose up --build
```

The flag `-d` can be used to run the container in the background, in which case
you can tear it down by running

```sh
docker compose down
```

Environment variables can be specified in an `.env` file (if it is declared in
the service configuration in the `docker compose.yml` file).
The docker compose services expect the `APP__DATABASE__PASSWORD` variable to be set.

## Testing

Postgres must be listening before running `cargo test`. For example

```sh
docker compose up -d postgres
cargo test
```

## Minimum supported Rust version

ChocoAPI's MSRV is 1.62.
