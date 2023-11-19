use rand::Rng;
use std::cmp::Ordering;
use std::io;

// If we _need_ our value to be within a certain range, or follow some other arbitrary criteria,
// instead of passing an i32 around all functions, and having functions either assume the value is
// valid or have to check it each call, we can create a custom type! Our example Guess type is
// basically a wrapper around an i32, but guarantees it to be valid within the appropriate range:
pub struct Guess {
    value: i32, // Private field! Can only be read with the value() method.
}

impl Guess {
    pub fn new(value: i32) -> Guess { // Ctor
        if value < 1 || value > 10 {
            panic!("Guess value must be between 1 and 10, got {value}!");
        }

        return Guess { value };
    }

    pub fn value(&self) -> i32 { // Getter
        return self.value;
    }
}

fn main() {
    println!("Guess the number!");

    // 0..2 --> Range [0, 2)
    // 0..=2 --> Range [0, 2]
    let number = rand::thread_rng().gen_range(1..=10);
    println!("((Shh, you're not supposed to know this, but the number is {number}))");

    let mut guess = String::new();

    loop {
        println!("Please input your guess.");

        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess_number: u32 = guess.trim().parse().expect("Not a valid number!");
        let guess_number: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number!");
                continue;
            }
        };

        if guess_number < 1 || guess_number > 10 {
            println!("The secret number is between 1 and 10.");
            continue;
        }

        println!("You guessed: {guess_number}");

        match guess_number.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
