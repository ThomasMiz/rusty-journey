// Associated types on a trait are similar to generics, but one key difference is that
// there can only be one implementation, which must specify what the associated type
// is, instead of having the possibility of multiple implementations for different
// types.

// In other words, when a trait has a generic parameter, it can be implemented for a
// type multiple times, and when we call `.next()` we'd have to specify for which
// type we're calling.

// This also means we don't need to annotate the type on each implementation.

// `Iterator` is defined as:
/*
pub trait Iterator {
    /// The type of the elements being iterated over.
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
    [...]
*/

// This means that if we want to implement this for a type named `Counter`:
/*
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> { [...] }
*/

// If `Iterator instead used generics, it'd look like this`:
/*
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
*/

// And there could be different implementations for counter, with different `T`s.

pub trait MyIterator<T> {
    fn next(&mut self) -> Option<T>;
}

pub struct Counter {
    current: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter {
            current: 0,
            max: max,
        }
    }
}

impl MyIterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        if self.current >= self.max {
            None
        } else {
            self.current += 1;
            Some(self.current)
        }
    }
}

impl MyIterator<String> for Counter {
    fn next(&mut self) -> Option<String> {
        if self.current >= self.max {
            None
        } else {
            self.current += 1;
            Some(self.current.to_string())
        }
    }
}

fn main() {
    let mut iter = Counter::new(5);
    print!("Counting up!");
    // If we use just `iter.next()` alone wouldn't work, because it doesn't know if
    // it's for `String` or `u32`. We must explicitly specify the type.
    while let Some(value) = <Counter as MyIterator<u32>>::next(&mut iter) {
        print!(" {value}");
    }
    println!();
    // Counting up! 1 2 3 4 5

    main2();
}

// This is useful for _operator overloading_:

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

use core::fmt;
use std::ops::Add;

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// `Add` actually has a generic parameter, `Rhs`, but it's `Self` by default. It looks
// like so: `pub trait Add<Rhs = Self>`. We can set it to something else to allow adding
// up `Point` with other types:
impl Add<i32> for Point {
    type Output = Point;

    fn add(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

fn main2() {
    let first = Point { x: 3, y: 4 };
    let second = Point { x: 5, y: -8 };
    let sum = first + second;
    println!("({}, {}) + ({}, {}) = ({}, {})", first.x, first.y, second.x, second.y, sum.x, sum.y);
    // (3, 4) + (5, -8) = (8, -4)

    let sum = first + 2;
    println!("({}, {}) + 2 = ({}, {})", first.x, first.y, sum.x, sum.y);
    // (3, 4) + 2 = (5, 6)

    main3();
}

// Different traits might define a method with a common name, and if a type implements
// two of these traits, or the type also has an impl block defining a method with that
// same name, we'll have to be more precise when calling these methods, as to eliminate
// the ambiguity.

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("!!ג'יבריש שולחן הקסם של מינכרפת");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously* I'M TRYING, MOM");
    }
}

fn main3() {
    let pablo = Human {};
    pablo.fly(); // By default, this calls the type's impl block, and not any trait's.
    // *waving arms furiously* I'M TRYING, MOM

    // We can disambiguate calls to the other `fly()` methods like so:

    Pilot::fly(&pablo);
    // This is your captain speaking

    Wizard::fly(&pablo);
    // !!ג'יבריש שולחן הקסם של מינכרפת

    Human::fly(&pablo); // Same as `pablo.fly()`
    // *waving arms furiously* I'M TRYING, MOM

    main4();
}


// What about non-method associated functions? They can also have this ambiguity:
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot") // Default name for a dog
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy") // Word to describe a baby dog
    }
}


fn main4() {
    // If we now call `Dog::baby_name()`, which should be called?
    // By default, the method from the struct's impl block:
    println!("The default name for a baby dog is {}", Dog::baby_name());
    // The default name for a baby dog is Spot

    // To call the method from `impl Animal for Dog`, we disambiguate like so:
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    // A baby dog is called a puppy

    main5();
}

// You can define a trait that depends on another trait. As in, for a type to implement
// the first trait, you want to require it also implements the second trait. This lets
// the first trait's definition make use of the associated items of the second trait.
// The second trait, the one your definition is relying on, is then called a
// _supertrait_ of your trait.

// In this example, we'll make an `OutlinedPrint` trait to print items in a '*' box:
trait OutlinedPrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// Now `OutlinedPrint` can only be implemented for types that also implement `Display`.
impl OutlinedPrint for Point {}

// That won't work unless we implement `Display` for `Point`!
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main5() {
    let p = Point { x: 5, y: 4 };
    p.outline_print();
    // **********
    // *        *
    // * (5, 4) *
    // *        *
    // **********

    main6();
}

// The "orphan rule" we saw back in chapter 10 states that we're only allowed to
// implement a trait for a type if either the trait or the type is local to our crate.
// We can _kinda_ get around this restriction using the _newtype pattern_, where we
// create a simple tuple struct that will have one field and simply be a wrapper
// around the target type. Since this new type is local to our crate, we can now
// implement any trait we want on it.

// This pattern originates from Haskell. Note that there is no runtime performance
// penalty, as the wrapper type is elided at compile time.

// For example, let's make a new `Display` implementation for `Vec<String>`:
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// This allows us to seamlessly deref a `Wrapper` into a `Vec<String>`, so we can
// use `w.len()` instead of `w.0.len()`.
use std::ops::Deref;
impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

fn main6() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("A vector of len {}! It says: {}", w.len(), w);
    // A vector of len 2! It says: [hello, world]
}
