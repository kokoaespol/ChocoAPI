# ChocoAPI

API/Backend for projects of Kokoa club

## Requirements

- rustup
- lld
- clang
- docker && docker-compose
- cargo install sqlx
- postgres running (use scripts/init_db.sh)
- Copy .env.sample to .env
- use `cargo sqlx prepare --check -- --bin chocoapi`

## Running

The application can be run using docker-compose:

```sh
docker-compose up
```

That will spawn a docker-compose instance running the api container with
postgres and redis as services. If the api container is not already built, that
command will also build it the first time, but to rebuild it after changing the
code you'll need to use the `--build` flag:

```sh
docker-compose up --build
```

The flag `-d` can be used to run the container in the background, in which case
you can tear it down by running

```sh
docker-compose down
```

Environment variables can be specified in an `.env` file (if it is declared in
the service configuration in the `docker-compose.yml` file)

To run any service standalone you can issue the following command:

```sh
docker-compose run <service-name>
```

For example:

```sh
docker-compose run --service-ports postgres
```

Note the `--service-ports` flag here. It is needed to map the container ports
to the host ports and be able to connect to the service.

## Environment setup

The docker-compose services expect the `APP_ENVIRONMENT` variable to be set.

Currently app configuration is done through the configuration files in the
`/configuration` directory and can be overwritten locally with an `.env` file.
Copy the `.env.sample` file removing the `.sample` extension and modify as needed.
