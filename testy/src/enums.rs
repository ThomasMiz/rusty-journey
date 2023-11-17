
/*enum IpAddrKind {
    V4,
    V6,
}*/
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
// The standard library's definition of IpAddr uses V4(Ipv4Addr) and V6(Ipv6Addr), those being structs that
// represents addresses in their respective formats.

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Enums can have associated functions too!
impl Message {
    fn call(&self) {
        println!("Who're you gonna call?");
    }
}

#[derive(Debug)]
enum UsState {
    Alabana,
    Alaska,
    // etc
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    return match coin {
        // Each arm of the match is a pattern, a => operator, then the corresponding code/expression.
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("I found a quarter from state {state:?}");
            25
        }
    };
}

fn do_i_like_coin(coin: Coin) -> bool {
    return match coin {
        Coin::Dime => true,
        _ => false, // Catch-all, discards variable
    }
}
// NOTE: Catch-alls MUST be at the end of the match, because the arms are evaluated in the order they are specified!

fn do_i_like_coin_shouting(coin: Coin) -> bool {
    return match coin {
        Coin::Dime => true,
        other => { // Catch-all, but gives us the variable
            println!("I DO NOT LIKE COINS THAT ARE {other:?}");
            false
        }
    }
}
// Want a match arm that matches things but doesn't do anything? Return a unit! For example: _ => ()

fn plus_one(x: Option<i32>) -> Option<i32> {
    return match x {
        Some(num) => Some(num + 1),
        None => None,
    };
}

fn one_if_some<T>(x: Option<T>) -> i32 {
    return match x {
        None => 0,
        Some(_) => 1
    };
}

fn main() {
    /*let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;*/

    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("value"));

    let msg = Message::Write(String::from("string strang"));
    msg.call();
    // Who're you gonna call?


    // Rust doesn't have null, but we can represent the concept of "value is invalid or not present" using the
    // Option<T> enum, which has variants None and Some(T):
    let maybe_string: Option<String> = Option::None;
    println!("maybe_string: {maybe_string:?}");
    // maybe_string: None

    // Note: Option<T> and its variants are included in the prelude, you may skip the "Option::" part:
    let maybe_string = Some(String::from("value"));
    println!("maybe_string: {maybe_string:?}");
    // maybe_string: Some("value")

    match maybe_string {
        Some(s) => println!("maybe_string contains a string! {s}"),
        None => println!("maybe_string doesn't contain a string."),
    }
    // maybe_string contains a string! value


    let maybe_string = Some(String::from("value"));
    // This:
    match maybe_string {
        Some(ref s) => println!("The string is {s}"),
        _ => ()
    }
    // is the same as this:
    if let Some(ref s) = maybe_string {
        println!("The string is {s}");
    }
    // NOTE: The "ref"s are because otherwise maybe_string is `moved` in the match's first arm. With the ref,
    // we ensure the arm borrows the value instead of getting ownership of it. We could have alternatively
    // matched &maybe_string instead.
}
