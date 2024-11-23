# How to start

In order to run this example, you will need to set up and migrate the database first.

# Prerequisites

## Tools and Environment

- sqlx-cli

```sh
cargo install sqlx-cli
```

- `.env` file with least those variables in it:

```txt
DATABASE_URL=sqlite://pizza.db
SERVING_IP_DEV=127.0.0.1
TYPE=dev
```

## Creating and seeding the database

```sh
sqlx database create
sqlx migrate run
```

## Run the server

```sh
cargo run (--release)
```