# Verbose Macros

[![license-mit](https://img.shields.io/badge/License-MIT-teal.svg)](https://opensource.org/license/mit/)
[![ci state](https://github.com/veeso/verbose-macros/actions/workflows/ci.yml/badge.svg)](https://github.com/veeso/verbose-macros/actions/workflows/ci.yml)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org)

[![Rust crate](https://img.shields.io/crates/v/verbose-macros.svg)](https://crates.io/crates/verbose-macros)
[![Rust documentation](https://docs.rs/verbose-macros/badge.svg)](https://docs.rs/verbose-macros)

A dependency-free macro library for **verbose!** and **debug!** printing in Rust.

## Introduction

I usually use `log` with all log levels for printing, but some people prefer to use just the `--verbose` and `--debug` flags.

For this reason, I created this library to provide a simple way to print verbose and debug messages without the need for any dependencies or setup to the `log` crate.

The library just provides the `verbose!` and `debug!` macros, which have the same syntax as `println!` and prints only if the flags for them are set.

## Get Started

Add the following to your `Cargo.toml`:

```toml
[dependencies]
verbose-macros = "0.1"
```

Then, you can use the macros in your code:

```rust
use verbose_macros::{debug, verbose};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let verbose = std::env::args().any(|arg| arg == "--verbose" || arg == "-v");
    let debug = std::env::args().any(|arg| arg == "--debug" || arg == "-d");

    // Set the debug and verbose flags
    verbose_macros::set_debug(verbose);
    verbose_macros::set_verbose(debug);

    // Use the debug macro
    debug!("This is a debug message.");
    debug!("This is a debug message with a value: {}", 42);

    // Use the verbose macro
    verbose!("This is a verbose message.");
    verbose!("This is a verbose message with a value: {}", 42);

    Ok(())
}
```

## License

This project is licensed under the [MIT License](LICENSE).
