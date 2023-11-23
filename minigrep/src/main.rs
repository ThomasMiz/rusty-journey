use std::{env, process};

use minigrep::Config;

fn main() {
    // Note: `env::args()` panics if any of the arguments contains invalid unicode. If you need
    // to accept args with invalid unicode use `std::env::args_os` instead.
    let args: Vec<String> = env::args().collect();
    let config_result = Config::build(&args);

    let config = config_result.unwrap_or_else(|err| {
        // Note the usage of `eprintln!` instead of `println!` to print to stderr
        eprintln!("{}", err);
        process::exit(1);
        // Look at the signature of this function: `fn exit(code: i32) -> !`
        // The `!` indicates that _the function does not return_, and therefore we don't get an
        // error indicating that this closure should be returning a `Config`.
    });

    let result = minigrep::run(config);
    if let Err(e) = result {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
