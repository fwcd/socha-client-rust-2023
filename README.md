# Software Challenge 2023 Rust Client

[![Build](https://github.com/fwcd/socha-client-rust-2023/actions/workflows/build.yml/badge.svg)](https://github.com/fwcd/socha-client-rust-2023/actions/workflows/build.yml)

A template client for the [Software Challenge 2023](https://www.software-challenge.de) (Ostseeschach) written in Rust.

The client implements [the XML protocol](https://docs.software-challenge.de/spiele/ostseeschach/xml-dokumentation.html) for communication with the game server, the game structures and a very simple logic that picks moves at random.

## Building

To build the client into a statically linked executable, run

```bash
cargo build --release
```

## Running

First make sure to have the game server for "Hey, Danke für den Fisch!" running (you can [download a copy from GitHub here](https://github.com/software-challenge/backend/releases/tag/23.0.1)).

To start an instance of the client, you can now run

```bash
cargo run --release
```

> Note that you will need another client (either a second instance of this one or another one) to play.
