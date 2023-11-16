struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

#[derive(Debug)] // Make a default implementation for the Debug trait, allowing us to print with {:?}.
struct Rectangle {
    width: u32,
    height: u32
}

// All the functions in the impl block are called "Associated Functions". A struct can have mutliple impl blocks,
// this is the same as having just one though.
impl Rectangle {
    // All methods must have their first parameter be self: &Self. This can be abbreviated to just &self.
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    // Methods can borrow self immutably (&self), borrow mutably (&mut self), or take ownership (self).
    // The latter is pretty rare; it's typically used when the method transforms the struct into something
    // else and you don't want the caller to keep using the original instance after the transformation.

    // Methods can have the same name as one of the struct's fields:
    fn width(&self) -> bool {
        return self.width > 0;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return other.width < self.width && other.height < self.height;
    }

    // Doesn't start with self!
    fn square(size: u32) -> Self { // Note: "Self" can be replaced with "Rectangle" in here
        return Self { width: size, height: size }
    }
    // These are typically used to create constructors, often named "new". "new" isn't a special keyword in Rust.
}

fn build_user(email: String, username: String) -> User {
    // Create a user. The variable names email and username are implicitly put into the fields of the same name
    // using the field init shorthand. Note that email and username are not in the same order as declared in the struct.
    return User {
        active: true,
        email, // Same as email: email
        username, // Same as username: username
        sign_in_count: 1
    };
}

fn main() {
    println!("Pedro");

    let user = User {
        active: true,
        username: String::from("Pedro McPedro"),
        email: String::from("pedro@mcpedro.com"),
        sign_in_count: 0
    };
    println!("User: active={}, username={}, email={}, sign_in_count={}", user.active, user.username, user.email, user.sign_in_count);

    let user = build_user(String::from("pedro2@mcpedro.com"), String::from("Pedro 2"));
    println!("User: active={}, username={}, email={}, sign_in_count={}", user.active, user.username, user.email, user.sign_in_count);


    /*let user2 = User {
        active: user.active,
        username: user.username,
        email: String::from("another@email.com"),
        sign_in_count: user.sign_in_count
    };
    println!("User: active={}, username={}, email={}, sign_in_count={}", user2.active, user2.username, user2.email, user2.sign_in_count);*/

    // OR, since we're copying all but one fields of the user, we can use this notation:
    let user2 = User {
        email: String::from("another@email.com"),
        ..user
    };
    println!("User: active={}, username={}, email={}, sign_in_count={}", user2.active, user2.username, user2.email, user2.sign_in_count);


    let rect = Rectangle {
        width: 50,
        height: 30
    };
    // println!("Rectangle: {rect}"); // Won't work, because Rectangle doesn't implement the std::fmt::Display trait
    println!("Rectangle: {rect:?}"); // Works! Because we added the derive thingy to make it implement the Debug trait.
    // Rectangle: Rectangle { width: 50, height: 30 }

    println!("Rectangle: {rect:#?}"); // Having Debug, we can also print like this. This adds newlines, which helps printing big structs
    /*
    Rectangle: Rectangle {
        width: 50,
        height: 30,
    }
    */

    // You can debug print (with Debug trait, to stderr) using the dbg! macro:
    dbg!(&rect); // If we had passed rect instead of &rect, it would have been dropped. Unlike println!, dbg! takes ownership of the value.
    /*
    [src/main.rs:73] &rect = Rectangle {
        width: 50,
        height: 30,
    }*/

    // Debug returns the passed value, so it's great for printing while assigning:
    let scale = 5;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: dbg!(20 * scale)
    };
    // [src/main.rs:83] 30 * scale = 150
    // [src/main.rs:84] 20 * scale = 100

    println!("The area of rect2 is {}", rect2.area());
    // The area of rect2 is 15000


    println!("The structs width is {}, is that greater than 0? {}", rect2.width, rect2.width());
    // The structs width is 150, is that greater than 0? true

    println!("Can rect hold rect2? {} And rect2 hold rect? {}", rect.can_hold(&rect2), rect2.can_hold(&rect));
    // Can rect hold rect2? false And rect2 hold rect? true

    let sqr = Rectangle::square(50);
    println!("Look at my square! {:?}", sqr);
    // Look at my square! Rectangle { width: 50, height: 50 }
}
