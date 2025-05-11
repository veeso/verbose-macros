//! A dependency-free macro library for verbose and debug printing in Rust.
//!
//! ## Introduction
//!
//! I usually use `log` with all log levels for printing,
//! but some people prefer to use just the `--verbose` and `--debug` flags.
//!
//! For this reason, I created this library to provide a simple way to print verbose and debug messages
//! without the need for any dependencies or setup to the `log` crate.
//!
//! The library just provides the `verbose!` and `debug!` macros, which have the same syntax as `println!`
//! and prints only if the flags for them are set.
//!
//! ## Getting Started
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! verbose-macros = "0.1"
//! ```
//!
//! Then, you can use the macros in your code:
//!
//! ```rust
//! use verbose_macros::{debug, verbose};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let verbose = std::env::args().any(|arg| arg == "--verbose" || arg == "-v");
//!     let debug = std::env::args().any(|arg| arg == "--debug" || arg == "-d");
//!
//!     // Set the debug and verbose flags
//!     verbose_macros::set_debug(verbose);
//!     verbose_macros::set_verbose(debug);
//!
//!     // Use the debug macro
//!     debug!("This is a debug message.");
//!     debug!("This is a debug message with a value: {}", 42);
//!
//!     // Use the verbose macro
//!     verbose!("This is a verbose message.");
//!     verbose!("This is a verbose message with a value: {}", 42);
//!
//!     Ok(())
//! }
//! ```

use std::sync::OnceLock;

static DEBUG: OnceLock<bool> = OnceLock::new();
static VERBOSE: OnceLock<bool> = OnceLock::new();

/// A macro for debug printing.
///
/// It prints the message only if the debug flag is set.
///
/// The debug flag is set by calling [`set_debug`] with `true`.
///
/// # Examples
///
/// ```rust
/// use verbose_macros::{debug, set_debug};
/// let name = "World";
/// set_debug(true);
/// debug!("Hello, {name}!");
/// debug!("This is a debug message.");
/// debug!("This is a debug message with a value: {}", 42);
/// ```
#[macro_export]
macro_rules! debug {
    ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($arg:tt)+) => ({
        if $crate::is_debug() {
            println!("{}", format!($($arg)+));
        }
    });

    ( $($arg:tt)+) => ({
        if $crate::is_debug() {
            println!("{}", format!($($arg)+));
        }
    });
}

/// A macro for verbose printing.
///
/// It prints the message only if the verbose flag is set.
///
/// The verbose flag is set by calling [`set_verbose`] with `true`.
///
/// # Examples
///
/// ```rust
/// use verbose_macros::{verbose, set_verbose};
///
/// let name = "World";
/// set_verbose(true);
/// verbose!("Hello, {name}!");
/// verbose!("This is a verbose message.");
/// verbose!("This is a verbose message with a value: {}", 42);
/// ```
#[macro_export]
macro_rules! verbose {
    ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($arg:tt)+) => ({
        if $crate::is_verbose() {
            println!("{}", format!($($arg)+));
        }
    });

    ( $($arg:tt)+) => ({
        if $crate::is_verbose() {
            println!("{}", format!($($arg)+));
        }
    });
}

/// Set the debug flag.
///
/// # Panics
///
/// Panics if the debug flag is already set.
pub fn set_debug(value: bool) {
    DEBUG.set(value).expect("DEBUG already set");
}

/// Set the verbose flag.
///
/// # Panics
///
/// Panics if the verbose flag is already set.
pub fn set_verbose(value: bool) {
    VERBOSE.set(value).expect("VERBOSE already set");
}

/// Check if the debug flag is set.
#[inline(always)]
pub fn is_debug() -> bool {
    DEBUG.get().copied().unwrap_or(false)
}

/// Check if the verbose flag is set.
#[inline(always)]
pub fn is_verbose() -> bool {
    VERBOSE.get().copied().unwrap_or(false)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_debug() {
        assert!(!is_debug());
        set_debug(true);
        assert!(is_debug());
        debug!("This is a debug message.");
        debug!("This is a debug message with a value: {}", 42);
    }

    #[test]
    fn test_verbose() {
        assert!(!is_verbose());
        set_verbose(true);
        assert!(is_verbose());
        verbose!("This is a verbose message.");
        verbose!("This is a verbose message with a value: {}", 42);
    }
}
