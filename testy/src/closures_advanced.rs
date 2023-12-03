// Function pointers allows us to treat functions as typed values. They use a special type
// called `fn(params) -> result` and implement all closure traits, `FnOnce`, `FnMut`, and
// `Fn`. This means that, when possible, it's better to write functions using these traits
// as they will able to accept both function pointers and closures.

// One usage of function pointers is for interfacing with other languages, as other langs
// might not have closures.

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is {answer}");



    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let mut statuses: Vec<Status> = (0u32..5).map(Status::Value).collect();
    statuses.push(Status::Stop);
    print!("Statuses:");
    statuses.iter().for_each(|x| print!(" {x:?}"));
    println!();
    // Statuses: Value(0) Value(1) Value(2) Value(3) Value(4) Stop

    main2();
}

// Function pointers and closures can also be returned by functions like so:

fn returns_closures(i: i32) -> impl Fn(i32) -> i32 {
    if i < 0 {
        |x| x + 1
    } else {
        |x| x + 2
    }
}

fn returns_funcs(i: i32) -> fn(i32) -> i32 {
    fn add_two(x: i32) -> i32 {
        x + 2
    }

    if i < 0 {
        add_one
    } else {
        add_two
    }
}

fn main2() {
    let thing1 = returns_funcs(-1);
    let thing2 = returns_funcs(1);
    println!("thing1(5)={} and thing2(5)={}", thing1(5), thing2(5));
    // thing1(5)=6 and thing2(5)=7

    let thing1 = returns_closures(-1);
    let thing2 = returns_closures(1);
    println!("thing1(5)={} and thing2(5)={}", thing1(5), thing2(5));
    // thing1(5)=6 and thing2(5)=7

    main3();
}

// However, this only works because the closures are identical and capture no state.
// What if they did capture state? Then we'd need to use `dyn` to call them dynamically.

fn returns_closures_with_state(i: i32, offset: &'static i32) -> Box<dyn Fn(i32) -> i32> {
    if i < 0 {
        Box::new(|x| x + 1 + *offset)
    } else {
        Box::new(|x| x + 2)
    }
}

fn main3() {
    let thing1 = returns_closures_with_state(-1, &0);
    let thing2 = returns_closures_with_state(1, &0);
    println!("thing1(5)={} and thing2(5)={}", thing1(5), thing2(5));
    // thing1(5)=6 and thing2(5)=7
}
