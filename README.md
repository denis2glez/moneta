<div align="center">
 <img src="https://user-images.githubusercontent.com/30119590/140493098-3d387075-3eac-4ccd-aece-84d7de24d364.png" alt="Project logo">
</div>

<h3 align="center"> <b>crypto-forex</b>: opening the cryptocurrency exchange to the forex market </h3>

---

>*"Most theoretical monetary economists, even those who specialised in Foreign Exchanges, made
>relatively little use of the lessons taught by earlier periods of Foreign Exchange history. It is
>no wonder that practical experts responsible for devising and executing Foreign Exchange policies
>so often failed to benefit by the lessons of the past. Much theoretical knowledge and practical
>experience acquired by our forerunners is being largely wasted by being allowed to fade into
>oblivion."*
><p align="right" style="font-size: smaller;"> <a href = "https://doi.org/10.1017/S0022050700060976">
>The History of Foreign Exchange</a></p>


## 📝 Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Deployment](#deployment)
- [Usage](#usage)
- [Built Using](#built_using)
- [Acknowledgments](#acknowledgement)
- [License](#license)

## 🧐 About <a name = "about"></a>

This project aims to provide an open currency exchange API that connects the traditional foreign
exchange market with cryptocurrencies. With this goal in mind,

- [`coin-market-cap`](./coin-market-cap/README.md) crate implements a Rust client of the
  CoinMarketCap API while using a data caching strategy based on a migrated SQL database. 
  
- [`binance`](./binance/README.md) crate implements a Rust client of the Binance Public Spot API 
  while using a data caching strategy based on a migrated SQL database.

- [`crypto-forex`](./crypto-forex/README.md) crate provides the core backend server that offers both
  a REST API and a high-performance RPC API, while consuming the migrated SQL database from
  `coin-market-cap`.

See the README file that accompanies each crate for details about the implementation.

⚠️ Software under active development, not for production use... yet!

## 🏁 Getting Started <a name = "getting_started"></a>

**Disclaimer:** I don't have any type of relationship with CoinMarketCap.

## Prerequisites

For development, you need a Unix-like environment for now. If you are running Windows, you can use
the Windows Subsystem for Linux ([WSL](https://docs.microsoft.com/en-us/windows/wsl/install)).

- Install `curl`, `git`.
- Install [Rust](https://www.rust-lang.org/tools/install).
- Install [Docker](https://docs.docker.com/get-docker/).
- Install [PostgreSQL](https://www.postgresql.org/download/) interactive terminal
  [`psql`](https://www.postgresql.org/docs/current/app-psql.html).
- Install SQLx's command-line utility for managing databases
  [`sqlx-cli`](https://crates.io/crates/sqlx-cli).
- Obtain an [API key](https://coinmarketcap.com/api/documentation/v1/#section/Quick-Start-Guide)
  from CoinMarketCap.

If you don't know how to install a prerequisite on your system, please check out the links provided
for each of them above.

## 🔧 Setup

Please note that we keep all settings locally in each crate to avoid unnecessary dependencies.
For details on how to configure a development environment, take a look at
[`coin-market-cap`](./coin-market-cap/README.md#setup) setup section and
[`crypto-forex`](./crypto-forex/README.md#setup) setup section, respectively.

### Arch Linux
If you are using Arch Linux or a derivative, you could install all the development dependencies by
running the following commands.
```sh
sudo pacman -S curl git rust docker postgresql
# Install sqlx-cli only for postgres
cargo install sqlx-cli --no-default-features --features postgres
```

### Debian
If you are using Debian or a derivative (e.g. Ubuntu, Linux Mint), it is recommended to install Rust
using the standard installation script. You could install all the development dependencies by running
the following commands.
```sh
# sqlx-cli needs libssl-dev
sudo apt install curl git docker postgresql-client libssl-dev
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Install sqlx-cli only for postgres
cargo install sqlx-cli --no-default-features --features postgres
```

## 🚀 Deployment <a name = "deployment"></a>

At this time it is possible to deploy `coin-market-cap` using one of the Docker
[recipes](./coin-market-cap/README.md#using_docker) in the crate. But the plan is to make the
full project deployment using Kubernetes (see the [roadmap](./crypto-forex/README.md#roadmap)).

## 🎉 Acknowledgements <a name = "acknowledgement"></a>

Thanks to all the developers of the libraries used throughout the project.

## 📝 License <a name = "license"></a>
This project is licensed under the [MIT](./crypto-forex/LICENSE) license.