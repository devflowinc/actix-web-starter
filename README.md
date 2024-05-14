# Trieve actix-web starter template

## Local Development Start Guide

### Install OS Dependencies 

`apt-get update -y && apt-get -y install pkg-config libssl-dev libpq-dev`

### Install Dockekr

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

## Setup the Generated Rust Client

1. `cargo run --features runtime-env --manifest-path actix-server/Cargo.toml --bin redoc_ci > ./generated-openapi-client/openapi.json`
2. `cd generated-openapi-client`
3. `npx @openapitools/openapi-generator-cli generate -i openapi.json -g rust -c ./openapi-generator.yaml -o ./ --skip-validate-spec`
