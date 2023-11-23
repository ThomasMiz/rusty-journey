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
            seasonal_fruit: String::from("Peaches"),
        };
    }
}

// If we make an enum public, all its variants are also public!
pub enum Appetizer {
    Soup,
    Salad,
}
