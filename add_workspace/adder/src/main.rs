use add_one;

fn main() {
    let n = 4;
    println!("Hello, world! {n} plus one is {}", add_one::add_one(n));

    let (x, y): (u128, u128) = (5, 3);
    println!("Hello again! {x} plus {y} equals {}", add_one::add(x, y));
}
