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

```
$ docker-compose up
```

That will spawn a docker-compose instance running the api container with
postgres and redis as services. If the api container is not already built, that
command will also build it the first time, but to rebuild it after changing the
code you'll need to use the `--build` flag:

```
$ docker-compose up --build
```

The flag `-d` can be used to run the container in the background, in which case
you can tear it down by running

```
$ docker-compose down
```

Environment variables can be specified in an `.env` file (if it is declared in
the service configuration in the `docker-compose.yml` file) or using the `-e`
flag like so:

```
$ docker-compose up -e APP_ENVIRONMENT=local
```

To run any service standalone you can issue the following command:

```
$ docker-compose run <service-name>
```

For example:

```
$ docker-compose run --service-ports postgres
```

Note the `--service-ports` flag here. It is needed to map the container ports
to the host ports and be able to connect to the service.

## Testing

To test the application you'll need to have the docker-compose instance
running.  Then you can run the tests with the following command:

```
$ APP_ENVIRONMENT=testing cargo test
```

A separate configuration profile is needed because the tests must connect to
the database through localhost, while the application running inside
docker-compose refers to it by its service name.

## Environment setup

The docker-compose services expect certain environment variables to be set:

**chocoapi:**

- `APP_ENVIRONMENT` can be one of "local", "testing" or "production"

**postgres:**

- `POSTGRES_PASSWORD`
- `POSTGRES_USER`
- `POSTGRES_DB`

These need to be the same as used to connect from the api.

Currently app configuration is done through the configuration files in the
`/configuration` directory and environment variables. Feel free to use
whichever method works best for you. As an example, a valid `.env` might look
like this:

```
APP_ENVIRONMENT=local
POSTGRES_PASSWORD=LOCALTESTINGxmhu5jVVwJ4sMlz7DAdKf0z4QPFY9Yc
POSTGRES_USER=postgres
POSTGRES_DB=chocodb
```
