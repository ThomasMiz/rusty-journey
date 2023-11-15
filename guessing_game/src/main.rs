use rand::Rng;
use std::cmp::Ordering;
use std::io;

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
        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number!");
                continue;
            }
        };

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
