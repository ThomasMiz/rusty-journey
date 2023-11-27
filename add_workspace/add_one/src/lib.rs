//! # Add One
//!
//! `add_one` kinda stupid because it doesn't do anything you can already do
//! in Rust without effort 👍

use std::ops::Add;

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = add_one::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// Adds two numbers of the same type
///
/// # Examples
///
/// ```
/// let x: u128 = 8;
/// let y: u128 = 2;
/// let answer = add_one::add(x, y);
///
/// assert_eq!(10, answer);
/// ```
pub fn add<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
