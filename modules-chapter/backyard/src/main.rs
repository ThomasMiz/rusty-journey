use crate::garden::vegetables::Asparagus;

pub mod garden;
// Tells the compiler to include the garden module: which the compiler will look for in garden.rs, or ./garden/mod.rs

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
