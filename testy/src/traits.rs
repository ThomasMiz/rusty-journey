// A trait defined functionality a particular type has and can share with other types.

use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;

    // We can also have a method with a default implementation:
    fn summarize(&self) -> String {
        return format!("(Read more from {}...)", self.summarize_author());
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implement the trait Summary for the type NewsArticle
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        return self.author.clone();
    }

    fn summarize(&self) -> String {
        return format!("{}, by {} ({})", self.headline, self.author, self.location);
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        return format!("@{}", self.username);
    }

    fn summarize(&self) -> String {
        return format!("{}: {}", self.username, self.content);
    }

    // Note: There's no way to call the default summarize() implementation for Tweet instead of the one in here.
    // If you overwrite the default implementation, you loose the default implementation.
}

impl Summary for i32 {
    fn summarize_author(&self) -> String {
        return "Bruh, I'm just an i32".to_string();
    }
}

// If this were a library, users of this library can implement the Summary trait on their own types.
// However, we can only implement a trait for a type if either the trait is local to our crate, or the type is.
// This means we can implement the std::fmt::Display trait on our Tweet, since Tweet is local to us, or we can
// implement Summary on a Vec<T>, because Summary is local to us. But we cannot implement Display on Vec<T>, as
// neither of those is local to us.
// This rule prevents other code from breaking your code, and prevents a situation in which a type could impl
// a trait twice, for example if a user of our crate implemented Summary for Tweet. We already have an impl
// of that trait for that type! Which one should Rust use then?
// This restriction is called the _orphan rule_, and is part of a property called _coherence_.

// We can specify on function parameters that the type must impl a trait:
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// But what type is item? `&impl MyTrait` is actually syntax sugar for a generic that implements `MyTrait`!
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Note: These two are different!
// - `notify(item1: &impl Summary, item2: &impl Summary)`
// - `notify<T: Summary>(item1: &T, item2: &T)`
// Because the second one forces item1 and item2 to be the same type, while the first one does not.
// This is equivalent to the first one:
// - `notify<T: Summary, U: Summary>(item1: &T, item2: &U)`

// We can specify multiple traits using the + syntax:
pub fn notify3(item: &(impl Summary + Display)) {
    println!("Breaking legs! {item} {}", item.summarize());
}

// Or on generic types:
pub fn something<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {
    println!("The T is {t} and if I clone it it's {}", t.clone());
    println!("((Debug printing u: {u:?} {:?}", u.clone());
}

// Or an alternative, less cluttered syntax for that (that does the same thing) is:
pub fn something2<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("The T is {t} and if I clone it it's {}", t.clone());
    println!("((Debug printing u: {u:?} {:?}", u.clone());
}

// We can also return something that impls a trait!
fn get_next_summarizable() -> impl Summary {
    return Tweet {
        username: String::from("Sócrates"),
        content: String::from("Alexander fuck off pls you're coverign my sun"),
        reply: false,
        retweet: false,
    };
}
// This function returns a Tweet, but the calling code doesn't know that, it just knows that whatever it is,
// it implements Summary. However, the function behind the scenes still returns a Tweet. You can't make it
// sometimes return a Tweet and sometimes a NewsArticle. It can only return ONE type. From the perspective of
// the caller, the function returns a type that implements Summary, but only the compiler knows that type.

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        return Self {x, y};
    }
}

// We can make an impl with methods available just for Pair<T>s where T implements certain traits. The
// cmp_display method will only be available on pairs where T implements both Display and PartialOrd:
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Lastly, we can implement a trait for all types that implement another trait. For example, the standard
// library implements the ToString trait for all types that implement the Display trait like so:
/*
impl<T: Display> ToString for T {
    // [...]
}
*/
// This means that i32, String, and many others, get ToString automatically just by having Display! So we
// can call .to_string() on them.

fn main() {
    let tweet = Tweet {
        username: "Pablo".to_string(),
        content: "Pedro es un boludo".to_string(),
        reply: false,
        retweet: false,
    };
    println!("Tweet: {}", tweet.summarize());
    // Tweet: Pablo: Pedro es un boludo

    let article = NewsArticle {
        headline: "Cat sleeps on the carpet!".to_string(),
        location: "My room, right behind me".to_string(),
        author: "Thomas".to_string(),
        content: "Nina The Cat is calmly sleepign on the carpet behind me, enjoying a slight breeze coming from the window.".to_string(),
    };
    println!("Article: {}", article.summarize());
    // Article: Cat sleeps on the carpet!, by Thomas (My room, right behind me)

    println!("We can summarize i32s! {}", 5.summarize());
    // We can summarize i32s! (Read more from Bruh, I'm just an i32...)

    notify(&5);
    // Breaking news! (Read more from Bruh, I'm just an i32...)
    notify2(&tweet);
    // Breaking news! Pablo: Pedro es un boludo

    let p = Pair::new(5, 10);
    p.cmp_display();
    // The largest member is y = 10
}
