use std::{
    fs::File,
    io::{self, ErrorKind, Read}, error::Error,
};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
// One difference the ? operator (the "error propagation" operator) has is that the error type is converted
// to the returned error type using the from function defined in the From trait. This means that if we want
// to return an `OurError` instead of an `io::Error`, we can just add an `impl From<io::Error> for OurError`
// and keep using the ? operator, it will convert the `io::Error` into an `OurError`!

// We can even chain methods after the ? operator to further simplify the function:
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    let mut username_file = File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// In reality, ? can be used not just for Result, but also for Option and any other type that implements
// `FromResidual`.

// This function finds, if there is, the ast char of the first line of a text:
fn last_char_of_first_line(text: &str) -> Option<char> {
    return text.lines().next()?.chars().last();
}

fn main2() {
    // By default, panic causes _unwinding_ of the stack, and Rust cleans up the data of all functions
    // in the stack. If you prefer a panic to immediately abort execution without cleaning up, add the
    // following lines to Cargo.toml:
    // [profile.release]
    // panic = 'abort'

    // You can ask Rust to show a full stack trace on a panic by setting the RUST_BACKTRACE variable to 1.
    // This can be done with `$ export RUST_BACKTRACE=1` or by specifying the variable on each cargo run
    // like so: `$ RUST_BACKTRACE=1 cargo run`.

    // This backtrace is not _fully_ complete! If you want the _real full trace_, all the way up to libc
    // and the _start label, specify RUST_BACKTRACE=full instead. This is very likely overkill though.
    // Note: When running on release mode, debug symbols are disabled.
    // panic!("La PUCHA");

    let greeting_file_result: Result<File, std::io::Error> = File::open("hello.txt");
    match greeting_file_result {
        Ok(mut file) => {
            println!("We have opened the file!");
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(size) => println!("File has a size of {size}, it says: {contents}"),
                Err(_) => println!("Failed to read contents :("),
            }
        }
        Err(err) => match err.kind() {
            ErrorKind::NotFound => println!("File not found!"),
            ErrorKind::PermissionDenied => println!("Permission denied!"),
            other => println!("Failed to open file! {other}"),
        },
    }

    // Another way to handle errors is with closures. Here's an example of "open file or create":
    /*
    use std::fs::File;
    use std::io::ErrorKind;

    fn main() {
        let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }
    */

    // Alternatively, we can call `.unwrap()` on a `Result` to get its value if it's the `Ok` variant, or
    // call the `panic!`` macro if it's the `Err` variant:
    let greeting_file = File::open("hello.txt").unwrap();

    // `.expect()` behaves the same way, but allows us to pass a string for the `panic!` macro:
    let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
}

// Main can return a `Result` too!
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("nonexistingfile.txt")?;

    Ok(())
}