# trade-match

trade-match is a low latency, in-memory order matching engine capable of vertically scaling to hundreds of thousands of transactions per second.

## Getting Started

### Install Rust and Cargo

Follow the instructions on the [official Rust website](https://www.rust-lang.org/learn/get-started) to install Rust and Cargo.
You can verify the installation by running:

```sh
rustc --version
cargo --version
```

### Clone the Project

Clone the repository:

```sh
git clone https://github.com/yourusername/market-matching-engine.git
```

## Unit Tests

### Running Unit Tests

To run the unit tests, use the following command:

```sh
cargo test
```

## Benchmarks

### Running Unit Tests

We use the criterion crate for benchmarking. To run the benchmarks, use the following command:

```sh
cargo bench
```

## WIP

### Planned Features

- Support for additional order types (market, stop limit)
- Multithreading/parallelization at the symbol level
- TCP/IP API for order entry
- Events API for match and cancel notifications
- Additional benchmarking and load testing
