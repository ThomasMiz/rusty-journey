// NOTE: "mod" is not like an import keyword. We should only do this once in our module tree (in this file).
// In other files, we can refer to these modules directly by their name, and use:: them.
mod front_of_house;
mod back_of_house;
pub use front_of_house::hosting::add_to_waitlist;

fn deliver_order() {}


pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Create a summer breakfast. Note that since the Breakfast struct has private fields, we can't instantiate
    // it from here! We _need_ to use an associated method.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("wheat");
    println!("I have a {} toast!", meal.toast);
}

// A better alternative to having to write the full path crate::front_of_house::hosting::add_to_waitlist() is to
// simply bring the whole front_of_house::hosting module into scope, then we can call add_to_waitlist directly:
use crate::front_of_house::hosting;
fn do_something_lol() {
    add_to_waitlist();
    // It's better to avoid writing code like this, because it's unclear where add_to_waitlist is defined. It looks
    // like it might be locally defined, but it's actually from another module.
}


// Note: use brings the whole module into scope, and any references to the using module can also access the used module!
mod customer {
    pub fn eat_at_restaurant() {
        super::add_to_waitlist();
        // Due to the use above, this is the same as crate::front_of_house::hosting::add_to_waitlist();
    }
}


// When bringing in definitions from other modules, it's more idiomatic to specify the full path:
use std::collections::HashMap;
fn my_map() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}


// Be careful when you bring into scope things from different modules with the same name!
use std::fmt;
use std::io;
fn my_function() {
    let a: fmt::Result = Ok(());
    let b: io::Result<()> = Ok(());
}


// Alternatively, we can rename these types by giving them an alias:
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;
fn my_function2() {
    let a: FmtResult = Ok(());
    let b: IoResult<()> = Ok(());
}


// We can even re-export names with a new alias by adding pub to a use!
pub use std::fmt::Result as NewResult;
// This is called _re-exporting_.


// When importing multiple things from the same module, we can use brackets. These two lines:
// use std::cmp::Ordering;
// use std::io;
// are the same as this one line:
// use std::{cmp::Ordering, io};


// These two lines:
// use std::io;
// use std::io::Write;
// Are the same as this one line:
// use std::io::{self, Write};


// What if I want to import everything within a module? Use the glob operator:
use std::collections::*;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
