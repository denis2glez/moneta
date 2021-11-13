# A CoinMarketCap API Rust client 

## Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Setup](#setup)
- [Usage](#usage)
- [Roadmap](#roadmap)

## About <a name = "about"></a>

According to [CoinMarketCap](https://coinmarketcap.com/api/documentation/v1/#section/Introduction):

>*"The CoinMarketCap API is a suite of high-performance RESTful JSON endpoints that are specifically
designed to meet the mission-critical demands of application developers, data scientists, and
enterprise business platforms."*

This library implements a Rust client of their API while using a data caching strategy based on a
migrated SQL database to avoid concerns with staying within the call credit and API throttling limits
of your subscription plan.

## Getting Started <a name = "getting_started"></a>

These instructions will get you a copy of the project up and running on your local machine for
development and testing purposes. See [deployment](../README.md#deployment) for notes on how to
deploy the project on a live system.

### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Docker](https://docs.docker.com/get-docker/)
- Install [PostgreSQL](https://www.postgresql.org/download/) interactive terminal
  [`psql`](https://www.postgresql.org/docs/current/app-psql.html).
- Install SQLx's associated command-line utility for managing databases
  [`sqlx-cli`](https://crates.io/crates/sqlx-cli).
- Obtain an [API key](https://coinmarketcap.com/api/documentation/v1/#section/Quick-Start-Guide)
  from CoinMarketCap.

#### Arch Linux
If you are using Arch Linux or a derivative, you could install all the dependencies by running the
following commands.
```sh
sudo pacman -S rust docker postgresql
# Install sqlx-cli only for postgres
cargo install sqlx-cli --no-default-features --features postgres
```

#### Debian
If you are using Debian or a derivative (e.g. Ubuntu, Linux Mint), it is recommended to install Rust
using the standard installation script. You could install all the dependencies by running the
following commands.
```sh
# sqlx-cli needs libssl-dev and curl is used next
sudo apt install docker postgresql-client curl libssl-dev
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Install sqlx-cli only for postgres
cargo install sqlx-cli --no-default-features --features postgres
```

## Setup <a name = "setup"></a>

To get a development environment running, first change your current working directory to
`coin-market-cap`.

```sh
cd coin-market-cap
```

Then, create and initialize the Docker container with the database by running the following script.

```sh
./scripts/init_db.sh
```

## Quick check

Quickly check the package and all of its dependencies for possible errors.
```sh
cargo check
```

## Build

To build the application on your host machine use

```sh
cargo build
```

## Run tests

Now you can run all the default tests

```sh
cargo test
```
or just a specific group of tests, by adding `-- <pattern>` to filter. For instance,

```sh
cargo test -- crypto_listing
```
would runt all tests related with the endpoint `listings_latest`. Note that some tests (e.g.
`fetch_crypto_listing`) actually consume the endpoint and are marked as `ignore`. Make sure to set
your API key first, otherwise these group of tests will fail. Then you can run them as follows:
 ```sh
cargo test -- --ignored
```

## Usage <a name = "usage"></a>

To run the application on your host machine

```sh
cargo run
```
or if you want to run an optimized artifact (i.e. release build)
```sh
cargo run --release
```
Of course, this assumes that your API key is set.

### Build and run using Docker <a name = "using_docker"></a>

Otherwise, we could build the application using one of the Docker recipes in the `docker` directory.
For instance, 
```sh
docker build --tag coin-market-cap_debian --file docker/Dockerfile.debian .
```
Then execute it using

```sh
docker run --network host coin-market-cap_debian
```
In case you choose some of the `musl`-based targets (i.e. `Dockerfile.alpine` or `Dockerfile.scratch`)
for performance-sensitive code, it is [currently](https://github.com/richfelker/mallocng-draft)
recommended to bring an alternative high-performance `malloc` implementation. You could use the
crate `jemallocator` that provides an allocator using [`jemalloc`](http://jemalloc.net) as a backend.
See [`Cargo.toml`](./Cargo.toml) for the default configuration used.


## Roadmap <a name = "roadmap"></a>

- [x] Add `map` module that consumes the endpoint `/v1/cryptocurrency/map`.
- [x] Add `listings/latest` module that consumes the endpoint `/v1/cryptocurrency/listings/latest`.
- [x] Add PostgreSQL database for caching.
- [x] Add Docker build recipes (see `docker` directory).
- [x] Setup CI/CD (use [GitHub Actions](https://github.com/actions-rs)).
- [x] Add code coverage reports (e.g. [CodeCov](https://codecov.io/))
- [ ] Use a tracing library for instrumentation (e.g `tracing` crate).
- [ ] Include some micro-benchmarks (e.g. `criterion`).
- [ ] Add Kubernetes deployment support (use [`minikube`](https://minikube.sigs.k8s.io/docs/),
    [`kubegres`](https://www.kubegres.io/doc/getting-started.html)).