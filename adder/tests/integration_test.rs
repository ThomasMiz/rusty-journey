use adder::{self, Rectangle, add_two};

mod common;

#[test]
fn some_integration_test() {
    common::setup();
    let smaller = Rectangle { width: 5, height: 5 };
    let larger = Rectangle { width: add_two(smaller.width), height: add_two(smaller.height) };
    assert!(larger.can_hold(&smaller));
}