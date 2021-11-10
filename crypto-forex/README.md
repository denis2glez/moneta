# `crypto-forex` core backend

## Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Usage](#usage)
- [Roadmap](#roadmap)

## About <a name = "about"></a>

This crate contains the core backend implementation of the project, aiming to be correct and
high-performance. Specifically, it consumes a migrated PostgreSQL database, while offering the
different *families* of endpoints to client applications. Check the [root readme](../README.md) file
to get an overview of the project.

## Getting Started <a name = "getting_started"></a>

These instructions will get you a copy of the project up and running on your local machine for
development and testing purposes. See [deployment](../README.md#deployment) for notes on how to
deploy the project on a live system.

### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Docker](https://docs.docker.com/get-docker/)
- Install `curl` and optionally [gRPCurl](https://github.com/fullstorydev/grpcurl#installation)
- A running database (see the [configuration](./config/base.yaml))

#### Arch Linux
If you are using Arch Linux or a derivative, you could install all the required dependencies by
running the following commands.
```sh
sudo pacman -S rust docker curl
```

#### Debian
If you are using Debian or a derivative (e.g. Ubuntu, Linux Mint), it is recommended to install Rust
using the standard installation script. You could install all the required dependencies by running
the following commands.
```sh
sudo apt install docker curl
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Setup <a name = "setup"></a>

This crate assumes that you have a populated database running, see the [configuration](./config/base.yaml).
One way to do this is by [setting up](./coin-market-cap/README.md#setup) and running the `coin-market-cap` application first. Just remember that this requires setting the API key.

For testing purposes, you could instead do

```sh
cd coin-market-cap
# Create the Docker container with the database
./scripts/init_db.sh
# Populate the database with test data
cargo test update_crypto_populate_db_100_items
```

Once the database is up, to get a development environment running, first change your current working directory to `crypto-forex`.

```sh
cd ../crypto-forex
```

Quickly check the package and all of its dependencies for possible errors.
```sh
cargo check -p crypto-forex
```

Now you could run all the tests.

```sh
cargo test -p crypto-forex
```
or just a specific group of tests, by adding `-- <pattern>` to filter. For instance,

```sh
cargo test -p crypto-forex -- <pattern>
```

## Usage <a name = "usage"></a>

To execute the `crypto-forex` binary on your development machine
```sh
cargo run
```
or if you want to run an optimized artifact (i.e. release build)
```sh
cargo run --release
```

After which a health check is always advised
```sh
curl -v http://127.0.0.1:8080/health_check
```

## Roadmap <a name = "roadmap"></a>

- [ ] Add a REST API to cover the base use case (contract using
  [`cmc-openapi`](https://github.com/denis2glez/cmc-openapi)).
- [ ] Explore the [Coinbase API](https://developers.coinbase.com/) instead
  (see [coinbase-pro-rs)](https://github.com/denis2glez/coinbase-pro-rs)).
- [ ] Implement a remote procedure call system (i.e. gRPC).
- [ ] Create unit and integration tests.
- [ ] Use a tracing library for instrumentation (e.g `tracing` crate).
- [x] Setup CI/CD (use [GitHub Actions](https://github.com/actions-rs), GitLab CI, etc.).
- [ ] Add code coverage reports (e.g. [CodeCov](https://codecov.io/))
- [ ] Add support for mocking (e.g. `fake`, `mockall`, `wiremock`).
- [ ] Include some micro-benchmarks (e.g. `criterion`).
- [ ] Add Kubernetes deployment support (use [`minikube`](https://minikube.sigs.k8s.io/docs/),
    [`kubegres`](https://www.kubegres.io/doc/getting-started.html)).