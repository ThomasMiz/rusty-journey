use front_of_house::hosting::add_to_waitlist;

use crate::back_of_house::Breakfast;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            super::do_something_stupid();
        }

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }

    fn do_something_stupid() {}
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    // Structs too, are by default private. Their fields too, are by default private!
    // Making a struct public doesn't make its fields public, they may be made public individually.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // Private field!
    }

    // Impls must be made public too!
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            return Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches")
            };
        }
    }

    // If we make an enum public, all its variants are also public!
    pub enum Appetizer {
        Soup,
        Salad
    }
}

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
