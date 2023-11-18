fn main() {
    //let v = Vec::<i32>::new();
    let v: Vec<i32> = Vec::new();
    // The vector is immutable! We can't push values into it. It's read-only.
    println!("We made a vector of length {} and capacity {}!", v.len(), v.capacity());
    // We made a vector of length 0 and capacity 0!


    let mut v = vec![1, 2, 3];
    // We can use the vec! macro to create a vector with the specific elements.
    println!("The vector is {v:?}");
    // The vector is [1, 2, 3]

    v.push(4);
    v.push(5);
    v.push(6);
    println!("And now it is {v:?}");
    // And now it is [1, 2, 3, 4, 5, 6]


    // Getting values form methods can be done with indexing, or with .get()
    // Careful! If we use indexing, the program will panic if the index is invalid!
    let third_element = &v[2]; // Of type &i32
    println!("The third element is {third_element}");
    // The third element is 3

    let fourth_element = v.get(4); // of type Option<&i32>
    // In here the program wont panic, we can handle the possible error.
    match fourth_element {
        Some(e) => println!("The fourth element is {e}"),
        None => println!("There is no fourth element!"),
    }
    // The fourth element is 5


    print!("Iterating through the vector:");
    // Note: we use &v and not v, as that would cause a _move_ of the vector. Alternatively, we can use v.iter()
    // and even chain iterator stuff like for example v.iter().enumerate().
    for e in &v {
        print!(" {e}");
    }
    println!(" That's it! {v:?}");
    // Iterating through the vector: 1 2 3 4 5 6 That's it! [1, 2, 3, 4, 5, 6]

    // Note: iterating over &v or v.iter(), instead of v, causes the e variable to be of type &i32 instead of i32.
    // This variable is still immutable though. If we want to make it mutable, we can use &mut v:
    for ele in &mut v {
        *ele += 1; // <-- Note the dereference
    }
    println!("I've added 1 to all elements! {v:?}");
    // I've added 1 to all elements! [2, 3, 4, 5, 6, 7]


    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Cornflower Blue")),
        SpreadsheetCell::Float(0.69)
    ];
    println!("{row:?}");
    // [Int(3), Text("Cornflower Blue"), Float(0.69)]





    // The `String` type is implemented as a `Vec<u8>` with some extra guarantees, restrictions and capabilities.
    let s = String::new();
    println!("The string is now {}empty.", if s.is_empty() {""} else {"not "});
    // The string is now empty.

    let s = "Hello!".to_string();
    println!("The string is now {}empty.", if s.is_empty() {""} else {"NOT "});
    // The string is now NOT empty.

    let mut s = String::from("Again?");
    println!("The string is now {}empty.", if s.is_empty() {""} else {"NOT "});
    // The string is now NOT empty.

    s.push(' '); // Append a single character
    s.push_str("Every"); // Append a string slice `&str`
    s.push(' ');
    s.push_str("fucking");
    s.push(' ');
    s.push_str("time.");
    println!("The string now says: {s}");
    // The string now says: Again? Every fucking time.


    let s1 = String::from("Hello,");
    let s2 = String::from(" World!");
    let s = s1 + &s2;
    // The + operator between strings is a bit tricky. This is concatenating s2 to s1, but the operator is taking ownership
    // of s1 and borrowing s2. This means that we no longer own s1 and therefore is now invalid! What's really happening is
    // that s2 is being appended to s1, then s1 is being returned.
    println!("s is '{s}' and s2 is '{s2}'");
    // s is 'Hello, World!' and s2 is ' World!'


    // Also note, the + operator takes in a `&str` as second parameter, but we're supplying a `&String`! No problem, the
    // compiler can _coerce_ the `&String` into a `&str`. `&s` is automatically transformed to `&s[..]`:
    let s_as_str: &str = &s;
    println!("s_as_str says: {s_as_str}");
    // s_as_str says: Hello, World!

    // This means that we can easily concat multiple values!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + " - " + &s2 + " - " + &s3 + "!";
    println!("{s}");
    // tic - tac - toe!



    // If we want to create a `String` from a format operation, we can use the `format!` macro:
    let s = format!("The cow says {}, the cat goes {}, meanwhile I eat {fruit}", "moo", "meow", fruit="apples");
    println!("The string says: {s}");
    // The string says: The cow says moo, the cat goes meow, meanwhile I eat apples.

    // If you want to concat literals into a `&'static str`, instead ose the `concat!` macro:
    let s: &str = concat!("Meow", " says ", "the cat");
    println!("{s}");
    // Meow says the cat


    // Strings in Rust cannot be indexed with [i] like vectors. This is because strings are valid UTF-8 values. The following
    // string looks like 12 characters, but has a length of 24, because all those non-ascii chars need two bytes to be
    // represented in the UTF-8 encoding:
    let hello = String::from("Здравствуйте");
    println!("Hello says '{hello}' and has length of {}", hello.len());
    // Hello says 'Здравствуйте' and has length of 24

    //let slice = &hello[0..3]; // This panics! As it's trying to cut the 'д' character is half! Not valid UTF-8!
    let slice = &hello[0..4]; // Works, because it cuts the string right between chars. Valid UTF-8!
    println!("The slice says {slice}");
    // The slice says Зд


    // If you want to iterate through a string, you should be specific about whether you want bytes or chars:
    print!("In chars:");
    for ele in hello.chars() {
        print!(" {ele}");
    }
    println!();
    // In chars: З д р а в с т в у й т е

    print!("In bytes: ");
    for ele in hello.bytes() {
        print!(" {ele}");
    }
    println!();
    // In bytes:  208 151 208 180 209 128 208 176 208 178 209 129 209 130 208 178 209 131 208 185 209 130 208 181





    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{scores:?}");
    // {"Blue": 10, "Yellow": 50}

    let team_name = String::from("Blue");
    let team_score = scores.get(&team_name).copied().unwrap_or(-1);
    // Note: `.get()` takes in a `&K` as parameter and returns an `Option<&V>`. We use `.copied()` to turn that `Option<&i32>`
    // into an `Option<i32>`, with the value copied (if not None), and then we unwrap it or take -1 if it doesn't exist.
    println!("The {team_name} team has a score of {team_score}");
    // The Blue team has a score of 10

    // We can iterate through all the values in the hash map using a for on `&scores` or `scores.iter()`:
    print!("The HashMap says:");
    for (key, value) in &scores {
        print!(" \"{key}\": {value}");
    }
    println!();
    // The HashMap says: "Blue": 10 "Yellow": 50

    // For Copy-able types, on insertion they are copied onto the hashmap. However for non-Copy-able types, such as String,
    // they are _moved_ onto the hashmap, therefore the hashmap becomes the owner of said value.
    // This means that after insert()-ing a variable into the hashmap, that variable becomes invalid! It has been moved!
    // If we insert references to values into the hashmap, those references must be valid for as long as the hashmap is valid.

    // Insert if not present & compute if not present:
    scores.entry(String::from("Blue")).or_insert(20);
    scores.entry(String::from("Blue")).or_insert_with(|| 123);
    println!("{scores:?} (Nothing's changed!)");
    // {"Yellow": 50, "Blue": 10} (Nothing's changed!)


    // The following code counts how many times each word appears in a string:
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
    // {"hello": 1, "world": 2, "wonderful": 1}




    // EXERCISE: Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
    // and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    let mut numbers = vec![5, -1, 443, 90, 123, 321, 4, 4, 4, 900];
    println!("Analyzing: {numbers:?}");

    numbers.sort_unstable();
    let median = if numbers.len() == 0 {
        0.0
    } else if numbers.len() % 2 == 0 {
        let halflen = numbers.len() / 2;
        (numbers[halflen - 1] + numbers[halflen]) as f64 / 2.0
    } else {
        numbers[numbers.len() / 2] as f64
    };

    let mut map = HashMap::new();
    numbers.iter().for_each(|x| {
        let count = map.entry(x).or_insert(0);
        *count += 1;
    });
    let max = map.iter().max_by_key(|(_, v)| *v);

    println!("The median is {median} and the mode is {max:?}");
    // Analyzing: [5, -1, 443, 90, 123, 321, 4, 4, 4, 900]
    // The median is 47.5 and the mode is Some((4, 3))
}
