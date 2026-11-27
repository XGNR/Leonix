# LEONIX Command Guide

LEONIX currently provides a small Rust command-line toolkit for local system
identification and repeatable CPU/memory micro-benchmark experiments.

## Build

Install the stable Rust toolchain, then run:

```sh
cargo build --release
```

## Commands

Show system information:

```sh
cargo run --release -- info
```

Run the default benchmark:

```sh
cargo run --release -- bench
```

Choose a CPU iteration count:

```sh
cargo run --release -- bench 5000000
```

Show help:

```sh
cargo run --release -- help
```

## What is measured?

- **System report:** operating system, CPU architecture, logical processor count,
  and current working directory.
- **CPU workload:** a deterministic integer-mixing loop.
- **Memory workload:** filling and scanning a bounded vector of 64-bit values.

The memory workload is capped to avoid unexpectedly large allocations. The
standard-library-only build does not claim to report physical RAM usage, GPU
details, or real game frame rates.

## Reproducibility

For more useful comparisons, run the same release build several times under
similar power and background-task conditions. Record the operating system,
hardware, Rust version, iteration count, and results. These micro-benchmarks
are educational research tools, not substitutes for testing an actual game.
