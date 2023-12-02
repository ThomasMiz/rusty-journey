fn main() {
    let x: Option<i32> = Some(5);
    match x {
        None => println!("None!"),
        Some(i) => println!("Some ({i})!"),
    }
    // Some (5)!
    // Note: match expressions must be _exhaustive_, as in, cover all possible values.



    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}!");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age { // <-- Shadowed variable!
        let color = if age > 30 {"purple"} else {"orange"};
        println!("We're using {color} then");
    } else {
        println!("Fuck it, we're going blue.")
    }
    // We're using purple then



    let mut stack = vec![1, 2, 3];
    print!("Printing this stacky stack:");
    while let Some(top) = stack.pop() {
        print!(" {top}");
    }
    println!("");
    // Printing this stacky stack: 3 2 1



    let v = vec!['a', 'b', 'c'];
    print!("Printing this little char vec");
    for (index, value) in v.iter().enumerate() {
        print!("; [{index}]={value}");
    }
    println!();
    // Printing this little char vec; [0]=a; [1]=b; [2]=c



    let (x, y, z, _) = (1, 2, 3, 4);
    println!("Tuple assignment! x={x}, y={y}, z={z}");
    // Tuple assignment! x=1, y=2, z=3



    // Pattern matching in a function's parameters!
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({x}, {y})");
    }

    let point = (3, 5);
    print_coordinates(&point);
    // Current location: (3, 5)



    // Patterns can be either refutable or irrefutable. Irrefutable ones will match for
    // any possible value, while refutable ones won't.

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("It be fifty"),
        Some(y) => println!("It be {y}"), // <-- Shadowed variable!!
        _ => println!("Uhhh I dunno but it's definitely not {y}"),
    }
    // It be 5

    // You can "or" patterns together:
    let x = 1;
    match x {
        1 | 2 => println!("One or two"),
        3 | 4 => println!("Three or four"),
        _ => println!("Somethin else, I guess")
    }
    // One or two

    // Or match against ranges:
    let x = 5;
    match x {
        1..=5 => println!("Greater than 0, but not greater than 5."),
        6..=10 => println!("Greater than 5, but not greater than 10."),
        _ => println!("Who the fuck knows")
    }
    // Greater than 0, but not greater than 5.

    // Ranges are also allowed for char values:
    let x = 'c';
    match x {
        'a'..='z' => println!("Lowercase ascii letter"),
        'A'..='Z' => println!("Uppercase ascii letter"),
        _ => println!("Somethin else (maybe it's a 💀)")
    }
    // Lowercase ascii letter


    // We can also break apart structs:
    struct Point { x: i32, y: i32 }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p; // Break apart the struct into `a` and `b`
    println!("a is {a} and b is {b}");
    // a is 0 and b is 7

    let Point { x, y } = p; // Shorthand for `let Point { x: x, y: y } = p;`
    println!("x is {x} and y is {y}");
    // x is 0 and y is 7

    // We can do the same in match arms, and specify that certain fields must match literals:
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }
    // On the y axis at 7


    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 }, // <-- Struct-like enum variant
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));
    match msg {
        Message::Quit => println!("Ya tryna quit"),
        Message::Move {x, y} => println!("Ya wanna move by x={x} and y={y}"),
        Message::Write(text) => println!("Texty text {text}"),
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!("Colory RGB color r={r} g={g} b={b}"),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!("Colory HSV color h={h} s={s} v={v}"),
    }
    // Colory color r=0 g=160 b=255



    // Example of pure blasphemy
    let ((feet, inches, _), Point { x, y }) = ((3, 10, "ignored string"), Point { x: 3, y: -10 });
    println!("feet={feet} inches={inches} x={x} y={y}");
    // feet=3 inches=10 x=3 y=-10



    // You can ignore parameters in a function
    fn foo(_: i32, y: i32) {
        println!("y is {y}, i forgor what x was 💀");
    }

    foo(3, 4);
    // y is 4, i forgor what x was 💀



    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing customized value"),
        _ => setting_value = new_setting_value,
    }
    // Can't overwrite an existing customized value

    println!("setting is {:?}", setting_value);
    // setting is Some(5)



    // This code won't compile:
    /*let s = Some(String::from("Hello!"));
    if let Some(_s) = s { // <-- `s` is moved into `_s`
        println!("Found a string!");
    }
    println!("s is {s:?}"); // <-- Attempt to use `s` after it has been moved
    */

    // We can avoid the issue of s being moved by _not binding_ a variable in the pattern:
    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("Found a string!");
    }
    // Found a string!
    println!("s is {s:?}");
    // s is Some("Hello!")



    // You can use `..` to ignore all remaining fields on a struct
    struct Point3D { x: i32, y: i32, z: i32 }
    let origin = Point3D { x: 0, y: 0, z: 0 };

    match origin {
        Point3D { x, .. } => println!("x is {x}"),
        // This is the same as `Point3D { x, y: _, z: _ } => println!("x is {x}"),`
    }
    // x is 0



    // `..` can also be used un tuples, in unambiguous cases:
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => println!("The first one is {first} and the last one {last}")
    }
    // The first one is 2 and the last one 32

    // Trying to do something like (.., second, ..) will not compile, as it's ambiguous.



    // _Match Guards_ are additional if conditions that can be speecified after a pattern in
    // a `match` arm, and the arm won't run unless the pattern also fulfils the condition:
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even!"),
        Some(x) => println!("The number {x} is odd!"),
        None => println!("There's no fucken number m8")
    }
    // The number 4 is even!

    // The downside of match guards is that the compiler can't check for exhaustiveness.

    // We can fix the issue with the shadowed variable we had before!
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("It be fifty"),
        Some(n) if n == y => println!("It be {y}"),
        _ => println!("Uhhh I dunno but it's definitely not {y}"),
    }
    // Uhhh I dunno but it's definitely not 10



    // Match guards can also be applied to patterns or-ed with `|`, and it will apply to
    // all of them:
    let x = 4;
    let y = false;
    match x {
        3 | 4 | 5 if y => println!("y is true AND x is 3, 4 or 5"),
        _ => println!("Maybe someday I'll stop typing `cargo ruin` instead of `cargo run`")
    }
    // Maybe someday I'll stop typing `cargo ruin` instead of `cargo run`



    // We can use the `@` operator to bind a variable to a value and check it against a
    // pattern at the same time:

    enum MyMessage {
        Hello { id: i32 },
    }
    let msg = MyMessage::Hello { id: 5 };

    match msg {
        MyMessage::Hello {
            id: id_variable @ 3..=7
        } => println!("Found an id in range! Its value is {id_variable}"),
        MyMessage::Hello { id: 10..=12 } => println!("Found id in another range, but I don't know its value :("),
        _ => ()
    }
    // Found an id in range! Its value is 5
}
