// If we want to use strongly-typed units, instead of using a plan i32 for measuring
// distance, we'd want to use a type that's not just a number, but also implicitly tells us
// this number means "meters", or "millimeters". A very simple way to do this is with a
// wrapper tuple struct:
// struct Millimeters(u32);
// struct Meters(u32);
// We can then impl traits like `Add<Meters> for Millimeters` and vice-versa, and we can
// implement conversions like `.as_millimeters()`.

// Another way to do this is with type aliases:
// type Kilometers = i32;
// This however, means `Kilometers` is just the same as `i32`. This does no prevent you
// from passing a `Millimeters` to a function that takes in `Meters` as before.

// This is why synonyms are typically used to reduce repetition, for example:
// type Thunk = Box<dyn Fn() + Send + 'static>;
// _thunk_ is a word for code to be evaluated at a later time, so it’s an appropriate name
// for a closure that gets stored.

// An example is `from std::io`: `pub type Result<T> = result::Result<T, Error>;` so
// they avoid having to type `Result<T, Error>` everwhere, as the error is the same.

fn main() {
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
    // x + y = 10
}

// A _diverging function_ is a function that never returns. We can mark these as returning
// type `!` like so:
fn bar() -> ! {
    loop {

    }
}

// _Dynamically sized types_, also known as _DSTs_ or _unsized types_, are types whose size
// is not known at compile time. One example is `str`, we can't know how long any string
// will be until we have it during runtime.

// We can't create variables of dynamically sized types. That means they also can't be used
// as function parameters, etc. That means that when we need them, we'll typically use
// references to them, such as `&str`.

// Note that traits are DSTs! That's why to use them as objects, we need to put them behind
// a pointer with `Box<dyn MyTrait>`.

// Sized types are indicated with the `Sized` marker trait. This trait is automatically
// implemented for every type whose size is known at compile time. Rust also adds a trait
// bound for `Sized` to all generic parameters on a function:

fn generic<T>(t: T) { // Same as `fn generic<T: Sized>(t: T)`

}

// You can relax this restriction with:
fn generic2<T: ?Sized>(t: &T) { // "`T` may or may not be sized"
    // Note: Since `T` might be dynamically sized, we can't return a `T`. We must put it
    // behind a pointer.
}
// Note: The `?Trait` syntax is only available for `Sized`, not for other traits.
