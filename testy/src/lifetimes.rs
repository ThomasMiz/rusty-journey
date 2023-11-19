fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// If we had made the function like this, it wouldn't have compiled:
// longest(x: &str, y: &str) -> &str
// Why? Because Rust doesn't know the lifetime of the returned type! Is it x? Or is it y? What if x is
// a static string, but y is stack-allocated? We solve this by specifying that the lifetime of the
// returned string is `'l`, which will be the minimum between the two.

// Lifetimes are specified using an aposthophe ('). For example:
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// In our `longest` function, we pass in two string slices that will live at least as long as `'a`. The
// returned value will have that lifetime `'a`.

// Here's an example of a function that takes in two string slices, x and y, then returns the x string
// but sliced to be no longer than y:
fn get_x_sliced_to_the_length_of_y<'a>(x: &'a str, y: &str) -> &'a str {
    if y.len() < x.len() {
        return &x[0..y.len()];
    }

    return x;
}
// See how we don't need to specify the lifetime of y! Just of x.

// If we want to add fields to a struct whose type is a reference, we need to specify lifetimes.
// This struct declaration is invalid:
// struct ImportantExcerpt {
//     part: &str,
// }
// To fix it, we need to add a lifetime to the reference:
struct ImportantExcerpt<'a> {
    part: &'a str,
}
// This ensures an instance of ImportantExcerpt can't outlive the references it holds.

// We don't always need to specify lifetimes. For example, take the following first_word function:
fn first_word(s: &str) -> Option<&str> {
    s.split_whitespace().next()
}
// The returned `&str` would have a lifetime as long as the `s` parameter. We didn't need to add the
// `<'a>` generic and the `'a` lifetime to the parameter and the return type! The compiler automatically
// figures that out. This didn't use to be like this, but since having to specify those lifetimes was so
// common and repetitive, Rust now detects these situations deterministically and infers the lifetimes.
// There analysis rules in the compiler ar ecalled _lifetime elision rules_.

// If we want to add methods to a struct with a generic lifetime parameter, we must use said parameter
// in the impl too:
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        return 3;
    }

    // We don't need to annotate lifetimes in this one thanks to the elision rules:
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        return self.part;
    }
}

// Finally, we have the static lifetime. This lifetime means the object exists for the whole duration
// of the program, and is never dropped. All literal strings, for example, have a static lifetime:
// let s: &'static str = "I have a static lifetime!";

use std::fmt::Display;

// One last example that brings together generic lifetimes, generic types, and traits:
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // This code fails, because it's attempting to store in `r` a value that lives less than `r`:
    /*let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);*/


    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(&string1, string2);
    println!("The longest string is {}", result);
    // The longest string is abcd

    let result = get_x_sliced_to_the_length_of_y("12345", "abc");
    println!("Result is {result}");
    // Result is 123

    let i = ImportantExcerpt {
        part: result
    };
    println!("Important! {}", i.part);
    // Important! 123
}
