// https://doc.rust-lang.org/book/appendix-01-keywords.html

// You can't normally use reserved keywords for variables or function names, but if you
// use the raw identifier `r#`, you can:

fn r#match(needle: &str, haystack: &str) -> bool {
    let r#if = haystack.contains(needle);

    r#if
}

// Raw identifiers allow you to use any name as identifier. This is mainly useful for
// interop with other languages where these words may not be keywords, or even other
// versions of Rust.

fn main() {
    let needle = "foo";
    let haystack = "foo bar";
    println!("Does {needle} contain {haystack} ? {}", r#match(needle, haystack));
    // Does foo contain foo bar ? true
}



// Operator overloading can be done by implementing specific traits:
// https://doc.rust-lang.org/book/appendix-02-operators.html


// Raw string literals, escape chars are not processed:
// r"..."
// r#"..."#
// r##"..."##,
// etc

// Byte string literals:
// b"..."

// Raw byte string literals, the previous two combined:
// br"..."
// br#"..."#
// br##"..."##

// Character literal: '.'
// ASCII byte literal: b'.'



// Derivable traits
// https://doc.rust-lang.org/book/appendix-03-derivable-traits.html
// Applied to any type with `#[derive(TraitName)]`
// `Debug`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Clone`, `Copy`, `Hash`, `Default`

// Note about `Copy`: it doesn't define any methods. It indicates that a value
// can be safely duplicated by copying the bits. No arbitrary code is run, so
// we can assume this copy operation is very fast.

// Note about `Hash`: Required for collections like `HashMap<K, V>`.

// Note about `Default`: Implements a `default()` function that allows getting a default
// value for a type. Useful, for example, for getting a `Vec` from a map or creating it
// if it's not in the map:
// let deparment_employees = map
//     .entry(String::from(department))
//     .or_default();
// This trait can be derived if all of a struct's types also implement `Default`. You
// can easily create an instance of a struct with a few customized fields and all others
// set to the default value with `{ field: my_value, ..Default::default() }`.



// Useful dev tools: https://doc.rust-lang.org/book/appendix-04-useful-development-tools.html
// Rust stable, beta & nightly: https://doc.rust-lang.org/book/appendix-07-nightly-rust.html
