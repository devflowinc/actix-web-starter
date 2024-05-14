# Trieve Actix Web Starter Template

## Local Development Start Guide

### Install OS Dependencies 

`apt-get update -y && apt-get -y install pkg-config libssl-dev libpq-dev`

### Install Docker

You can either install it the right way:

- https://docs.docker.com/engine/install/

or the fast way:

```
curl https://get.docker.com | sh
```

### Install Rust via Rustup
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install Diesel

`cargo install diesel_cli`

### Install Cargo Watch

`cargo install cargo-watch`

### Run the Dev Server

```
cd actix-server
cargo watch -x run
```

### Install the Local CLI

```
cd actix-template-cli
cargo install --path .
```

The CLI can also be run using the standard `cargo run --`, placing any arguments you want to send to the CLI after the "--".

## Setup the Generated Rust Client
Note: You must have Java installed on your system to run the openapi-generator-cli.

1. `cargo run --features runtime-env --manifest-path actix-server/Cargo.toml --bin redoc_ci > ./generated-openapi-client/openapi.json`
2. `cd generated-openapi-client`
3. `npx @openapitools/openapi-generator-cli generate -i openapi.json -g rust -c ./openapi-generator.yaml -o ./ --skip-validate-spec`
