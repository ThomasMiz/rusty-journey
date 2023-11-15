use std::num::IntErrorKind;

const BIG_ASS_NUMBER: i32 = 9000;

fn pedro(x: i32) -> i32 {
    return x * 2;
}

fn main() {
    let mut x: i32 = 42;
    x += 5;
    x = pedro(x);
    println!("Hello world! {}", x);
    // Hello world! 94

    let pair = ('a', "hola 👀");
    println!("Una tupla! Dice {1} y {0}.", pair.1, pair.0);
    // Una tupla! Dice a y hola 👀.

    let pair2: (char, i64) = ('b', 0xabcdef69420);
    println!("Una tupla con tipos :O {hola} {chau:x} {chau:X}", hola=pair2.0, chau=pair2.1);
    // Una tupla con tipos :O b abcdef69420 ABCDEF69420

    let triple: (f32, &'static str, u8) = (0.69420, "🍕🍕🍕", 128);
    println!("{0} {0:.5} {1} {2} {2:0>5}", triple.0, triple.1, triple.2);
    // 0.6942 0.69420 🍕🍕🍕 128 00128

    let (some_float, some_str, some_byte) = triple;
    println!("I repeat; {0} {0:.5} {1} {2} {2:0>5}", some_float, some_str, some_byte);
    // I repeat; 0.6942 0.69420 🍕🍕🍕 128 00128

    // Empty tuple, the "unit type". Expressions that don't return anything actually return ().
    let _unit_type = ();

    println!("Big ass number: {BIG_ASS_NUMBER}");
    // Big ass number: 9000

    let sss = "abcdefg";
    println!("The length of the string {sss} is {0}", sss.len());
    // The length of the string abcdefg is 7

    let i32_from_string: i32 = "123".parse().expect("Not a number! PROGRAM PANICKING!!!");
    println!("Look at my i32 from string! {i32_from_string}");
    // Look at my i32 from string! 123

    let i32_from_string_or_zero: i32 = match "1a23".parse() {
        Ok(value) => value,
        Err(s) => {
            println!("Error failed because: {}", match s.kind() {
                &IntErrorKind::Empty => "Empty",
                &IntErrorKind::Zero => "The string has a value of 0?",
                &IntErrorKind::InvalidDigit => "Invalid digit",
                &IntErrorKind::NegOverflow => "Negative overflow",
                &IntErrorKind::PosOverflow => "Positive overflow",
                _ => "Unknown error"
            });
            0
        },
    };
    // Error failed because: Invalid digit
    println!("Look at my i32 from string! (again) {i32_from_string_or_zero}");
    // Look at my i32 from string! (again) 0
    
    let i32_from_string_or_zero2: i32 = "9321a".parse().unwrap_or(0);
    println!("Look at my i32 from string! (again (again)) {i32_from_string_or_zero2}");
    // Look at my i32 from string! (again (again)) 0

    match "123".parse::<i32>() {
        Ok(value) => println!("The string's value as int is {value}, plus one is {0}", value + 1),
        Err(_) => println!("Not a valid string!"),
    };
    // The string's value as int is 123, plus one is 124

    // Data types: i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
    let big_number: u128 = 1_000_000_000_000_000_000_000;
    let same_big_number: u128 = 1000000000000000000000;
    let what_they_are = if big_number == same_big_number { "they are" } else { "they are NOT!!!" };
    println!("The same big ass number: {big_number}, {same_big_number}, they are the same (it's {}, {})", big_number == same_big_number, what_they_are);

    // The same big ass number: 1000000000000000000000, 1000000000000000000000, they are the same (it's true, they are)

    // By default, Rust checks for integer overflows on debug, but not on release.
    // You can disable checks by adding [profile.dev] overflow-checks = false to the Cargo.toml
    let pepe: u8 = 255;
    println!("pepe is {pepe}, plus one is {}", pepe + 1);
    // pepe is 255, plus one is 0


    let somechar = 'z';
    let anotherchar: char = '👀';
    println!("Somechar {somechar} anotherchar {anotherchar}");
    // Somechar z anotherchar 👀


    // STACK arrays with compile-time-constant fixed size. Copying them makes a copy of the whole array!
    let mut arr1 = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = arr1; // Array type is annotated as [type; length]
    arr1[0] = 3;
    println!("The first elements of both are {} {}", arr1[0], arr2[0]);
    // The first elements of both are 3 1

    let five = 5;
    let array_of_six_fives = [five; 6];
    println!("The array of six fives contains: {} {}, etc. Length of {}.", array_of_six_fives[0], array_of_six_fives[1], array_of_six_fives.len());
    // The array of six fives contains: 5 5, etc. Length of 6.

    let binary_number = 0b0100101010;
    let binary_number2 = 0b01_0010_1010;
    let octal_number = 0o77;
    let hex_number = 0x77;
    println!("Binary numbers: {:b} {:b} | Octal number: {:o} | Hexadecimal number: {:x} {:X}", binary_number, binary_number2, octal_number, hex_number, hex_number);

    let binary_char = b' '; // An u8!
    println!("Binary char: {binary_char}");
}
