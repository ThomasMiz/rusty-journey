fn main() {
    let number = 123;
    if number < 50 {
        println!("Number is less than 50");
    } else {
        println!("Number is more than 50")
    }
    // Number is more than 50

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divislbe by 2");
    } else {
        println!("I don't know, man");
    }
    // Number is divisible by 3

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The condition was {condition}, so the number is {number}.");
    // The condition was true, so the number is 5.


    loop { // Infinite loop
        println!("I'm in da loop 😎");
        break; // break the loop;
    }
    // I'm in da loop 😎


    // Loops are expressions, they can return results! You can retry an operation and return only the final result.
    // Add a value to the break expression to return it as the loop's value.
    let strings = ["Hello!", "This is not a number.", "Pedro", "-1234", "1234"];
    let mut strings_index = 0;
    let parse_number_result = loop {
        let parsed = strings[strings_index].parse::<u32>();
        match parsed {
            Ok(value) => { break value; },
            Err(_) => {
                println!("Failed to parse string: \"{}\"!", strings[strings_index]);
                strings_index += 1;
                continue;
            },
            // Any statement here would be unreachable, because the Ok() branch breaks the loop and the Err() branch uses continue.
        };
    };
    // Failed to parse string: "Hello!"!
    // Failed to parse string: "This is not a number."!
    // Failed to parse string: "Pedro"!
    // Failed to parse string: "-1234"!
    println!("Successfully parsed string \"{}\" to an u32 {} (or plus one equals {})!", strings[strings_index], parse_number_result, parse_number_result + 1);
    // Successfully parsed string "1234" to an u32 1234 (or plus one equals 1235)!


    // break and continue apply to the innermost loop at which they are placed. But what if I want to break an outer loop?
    // You can label loops with a single quote, and specify the quote to break and continue:
    let mut count = 0;
    'outer_loop: loop {
        println!("Start of outer loop with count={count}!");
        let mut count2 = 0;
        'inner_loop: loop {
            println!("Start of inner loop with count={count}!");
            count2 += 1;
            count += 1;
            if count > 4 {
                break 'outer_loop;
            }
            if count2 >= 3 {
                break 'inner_loop;
            }
        }
    }
    // Start of outer loop with count=0!
    // Start of inner loop with count=0!
    // Start of inner loop with count=1!
    // Start of inner loop with count=2!
    // Start of outer loop with count=3!
    // Start of inner loop with count=3!
    // Start of inner loop with count=4!



    let mut countdown = 10;
    print!("Counting to 10 before liftoff...");
    while countdown != 0 {
        countdown -= 1;
        print!("{} {countdown}", if countdown == 9 { "" } else { "," });
    }
    println!(" aaand LIFTOFF!");
    // Counting to 10 before liftoff... 9, 8, 7, 6, 5, 4, 3, 2, 1, 0 aaand LIFTOFF!


    // For loops can be used to iterate through a collection
    let some_arr = [1, 1, 2, 3, 5, 8, 13, 21];
    print!("The fibonacci sequence goes");
    for ele in some_arr {
        print!(" {ele}");
    }
    println!(" and so on");
    // The fibonacci sequence goes 1 1 2 3 5 8 13 21 and so on


    // Or if you want a "traditional" for loop, iterating through a range:
    println!("Counting from 0 to 9 for no reason:");
    for i in 0..10 {
        print!(" {i}");
    }
    println!(" done");
    // Counting from 0 to 9 for no reason:
    // 0 1 2 3 4 5 6 7 8 9 done


    println!("Now in reverse!");
    for i in (0..10).rev() {
        print!(" {i}");
    }
    println!(" done");
    // Now in reverse!
    // 9 8 7 6 5 4 3 2 1 0 done


    // Calculate the n-th Fibonacci number
    let n = 10;
    let mut prevprev = 0;
    let mut prev = 1;
    for _ in 2..n {
        let next = prev + prevprev;
        prevprev = prev;
        prev = next;
    }
    println!("The {n}-th Fibonacci number is {prev}");
}
