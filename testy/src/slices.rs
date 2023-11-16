fn push_world(s: &mut String) {
    s.push_str(", World!");
}

fn first_world(s: &str) -> &str {
    for (i, &byte) in s.as_bytes().iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

fn main() {
    let mut s = String::from("Hello");
    push_world(&mut s);
    println!("{s}");
    // Hello, World!

    for (i, &item) in s.as_bytes().iter().enumerate() {
        print!("[{i}]={item} ");
    }
    println!();
    // [0]=72 [1]=101 [2]=108 [3]=108 [4]=111 [5]=44 [6]=32 [7]=87 [8]=111 [9]=114 [10]=108 [11]=100 [12]=33


    let hello = &s[0..5];
    let world = &s[7..12];
    println!("{hello} {world}");
    // Hello World

    let hello2 = &s[..5];
    let world2 = &s[7..=11];
    let world_exclamation = &s[7..];
    let slice_of_everything = &s[..];
    println!("{hello2} | {world2} | {world_exclamation} | {slice_of_everything}");
    // Hello | World | World! | Hello, World!

    let hello = first_world(&s);
    // If you modify the s string here, compiler error!
    println!("The first world of {s} is {hello}");
    // The first world of Hello, World! is Hello,


    let array = [1, 2, 3, 4, 5, 6];
    let array_slice = &array[1..4]; // [2, 3, 4]
    assert_eq!(array_slice, &[2, 3, 4]);
}
