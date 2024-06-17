# trade-match

trade-match is a low latency, in-memory central limit order book capable of vertically scaling to hundreds of thousands of transactions per second.

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

## Usage

### Running Unit Tests

To run the unit tests, use the following command:

```sh
cargo test
```

### Running Benchmarks

We use the criterion crate for benchmarking. To run the benchmarks, use the following command:

```sh
cargo bench
```

## WIP

### Planned Features

- [x] Support for the limit order type
- [x] Support for the market order type
- [ ] TCP/IP based order entry API
- [ ] Order matched notifications
- [ ] Order cancelled notifications
- [ ] Multithreading/parallelization at the symbol level
- [ ] Memory pooling
- [ ] Additional benchmarking/stress testing
