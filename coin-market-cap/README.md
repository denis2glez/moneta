# A CoinMarketCap API Rust client 

## Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Usage](#usage)
- [Roadmap](#roadmap)

## About <a name = "about"></a>

According to [CoinMarketCap](https://coinmarketcap.com/api/documentation/v1/#section/Introduction):

>*"The CoinMarketCap API is a suite of high-performance RESTful JSON endpoints that are specifically
designed to meet the mission-critical demands of application developers, data scientists, and
enterprise business platforms."*

This library implements a Rust client of their API while using a data caching strategy based on a
local SQL database to avoid concerns with staying within the call credit and API throttling limits
of your subscription plan.

**Disclosure:** I don't have any type of relationship with CoinMarketCap.

## Getting Started <a name = "getting_started"></a>

These instructions will get you a copy of the project up and running on your local machine for
development and testing purposes. See [deployment](../README.md#deployment) for notes on how to deploy the
project on a live system.

### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Docker](https://docs.docker.com/get-docker/)
- Obtain an [API key](https://coinmarketcap.com/api/documentation/v1/#section/Quick-Start-Guide) from CoinMarketCap.

If you are using Arch Linux, you could install both dependencies by running:
```sh
sudo pacman -S rust docker
```

### Installing

To get a development environment running, first change your current working directory to `coin-market-cap`
```sh
cd coin-market-cap
```

Then, create the Docker container with the database by running the following script

```sh
./scripts/init_db.sh
```

Now you can run all the tests

```sh
cargo test -p coin-market-cap
```
or just a specific group of tests, by adding `-- <pattern>` to filter. For instance,

```sh
cargo t -p coin-market-cap -- cryptocurrency_listings_latest
```
would runt all tests related with the endpoint `listings_latest`. Note that some tests actually
consume the endpoint, so be sure to set your API key first, otherwise they will fail.

## Usage <a name = "usage"></a>

Add notes about how to use the system.

## Roadmap <a name = "roadmap"></a>

- [x] Add `map` module that consumes the endpoint `/v1/cryptocurrency/map`.
- [x] Add `listings/latest` module that consumes the endpoint `/v1/cryptocurrency/listings/latest`.
- [x] Add PostgreSQL database for caching.
- [x] Add Docker build recipe.
- [ ] Use a tracing library for instrumentation (e.g `tracing` crate).
- [ ] Include some micro-benchmarks (e.g. `criterion`).
- [ ] Add Kubernetes deployment support (use [`minikube`](https://minikube.sigs.k8s.io/docs/),
    [`kubegres`](https://www.kubegres.io/doc/getting-started.html)).