use verbose_macros::{debug, verbose};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let verbose = std::env::args().any(|arg| arg == "--verbose" || arg == "-v");
    let debug = std::env::args().any(|arg| arg == "--debug" || arg == "-d");

    // Set the debug and verbose flags
    verbose_macros::set_debug(debug);
    verbose_macros::set_verbose(verbose);

    // Use the debug macro
    debug!("This is a debug message.");
    debug!("This is a debug message with a value: {}", 42);

    // Use the verbose macro
    verbose!("This is a verbose message.");
    verbose!("This is a verbose message with a value: {}", 42);

    Ok(())
}
