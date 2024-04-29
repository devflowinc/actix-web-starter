# Trieve actix-web starter template

## Local Development Start Guide

This assumes you are using a debian linux distribution for local development and you have rust installed through rustup.

### Install OS Dependencies 

`apt-get update -y && apt-get -y install pkg-config libssl-dev libpq-dev`

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
