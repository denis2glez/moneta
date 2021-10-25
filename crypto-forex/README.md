# `crypto-forex` core backend

## Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Usage](#usage)
- [Roadmap](#roadmap)

## About <a name = "about"></a>

This crate contains the core backend implementation of the project, aiming to be correct and high-performance.
Specifically, it consumes a local migrated PostgreSQL database, while offering the different *families* of endpoints to client applications. Check the [root readme](../README.md) file to get an overview of the project.

## Getting Started <a name = "getting_started"></a>

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See [deployment](../README.md#deployment) for notes on how to deploy the project on a live system.

### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Docker](https://docs.docker.com/get-docker/)
- Install `curl` and optionally [gRPCurl](https://github.com/fullstorydev/grpcurl#installation)

### Installing

To get a development environment running, first change your current working directory to `crypto-forex`.
```sh
cd crypto-forex
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

To get a development environment running, execute
```sh
cargo run -p crypto-forex
```

After which a health check is always advised
```sh
curl -v http://127.0.0.1:8080/health_check
```

## Roadmap <a name = "roadmap"></a>

- [ ] Add a REST API to cover the base use case.
- [ ] Implement a remote procedure call system (i.e. gRPC).
- [ ] Add Kubernetes deployment support (use [`minikube`](https://minikube.sigs.k8s.io/docs/),
    [`kubegres`](https://www.kubegres.io/doc/getting-started.html)).