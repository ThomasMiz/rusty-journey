pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn panic_if_out_of_range(a: i32) {
    if a < 1 {
        panic!("OH NO THAT'S TOO LOW MAN");
    } else if a > 100 {
        panic!("SHIT BRO DAS TOO HIGH 💀");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 20, height: 20 };
        let smaller = Rectangle { width: 10, height: 10 };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let larger = Rectangle { width: 20, height: 20 };
        let smaller = Rectangle { width: 10, height: 10 };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        // assert_eq! and assert_ne! use `==` and `!=` respectively, so the arguments passed to these
        // macros must implement the `PartialEq` trait, and `Debug` for printing their values. Both of
        // these traits are derivable though! `#[derive(PartialEq, Debug)]`
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");

        //assert!(result.contains("Carol"));
        // The error message printed if this assertion fails not very useful. We can add a custom one
        // by using additional parameters on the `assert!` macro!

        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was {}",
            result
        );
    }

    #[test]
    #[should_panic]
    fn panics_when_zero() {
        panic_if_out_of_range(0);
    }

    // The issue with tests that should panic is that they might panic for a different reason than
    // you exepcted. To fix this, we can add an expected string for the panic reason:
    #[test]
    #[should_panic(expected = "TOO HIGH")]
    fn panics_when_200() {
        panic_if_out_of_range(200);
    }
    // If the test panics because of a too low value, then it will not be marked as passed like before!


    // Tests may also return a Result instead of panicking or not:
    #[test]
    // #[ignore] // If we want a test to be ignored unless specifically requested to run
    fn it_works2() -> Result<(), String> {
        if add_two(2) == 4 {
            Ok(())
        } else {
            Err(String::from("adding 2 to the number 2 did not equal 4"))
        }
    }
    // This allows us to make nicer tests, as we can use the error propagation operator `?` to do early
    // returns in case of failure.
    // Note: You can't use should_panic on tests that return results. If you want to check that an
    // operation returns an `Err` variant, use `assert!(value.is_err())` instead.
}

/*
When we run tests with `cargo test`, cargo will compile our code into a testing binary and then run it.
The default behavior of this binary is to run all tests, capture the output, and the summarize. But we
can change this behavior.

To see the options we can pass to cargo test: `cargo test --help`
To see the options we can pass to the test binary: `cargo test -- --help`
All arguments after the `--` are sent to the test binary.

Examples:
- Run the code in only one thread: `cargo test -- --test-threads=1`

- By default, only the tests that fail have their stdout printed to the terminal. If you want to print
    the stdout of all tests, use `cargo test -- --show-output`

- Only run tests whose name matches "it_works": `cargo test it_works`
    (In our case, this will run it_works and it_works2). Note that the name of each tests is it's full
    path, including the module name. That means we can run `cargo test tests::i` and it will run
    tests::it_adds_two, tests::it_works_ and tests::it_works2.

We can add `#[ignore]` to tests we don't want to run unless specifically specified. This is useful if,
    for example, we have a test that takes very long to run.
We can run only the ignored tests with `cargo test -- --ignored`
If you want to run all tests, whether ignored or not, use `cargo test -- --include-ignored`


Unit Testing
The convention is to add a tests module in each file, and to annotate this module with `#[cfg(test)]`,
    which tells Rust to compile and run the tests only when you run `cargo test`, not `cargo build`.


Integration Testing
Integration tests typically go in a separate `tests` directory, right next to `src`. Note that
    integration tests are not run if any unit tests fail.

You can run only the integration tests in the integration_test.rs file like so:
    `cargo test --test integration_test`


Something something submodules
If we wanted many of our integration tests to share something, like a setup() function, we could
    add a file to /tests/common.rs and place the `fn setup()` in there. However, this will cause
    common to appear in the test results, even if there are no tests in common.rs! To fix this,
    we can instead name this file /tests/common/mod.rs
*/