struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
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

}