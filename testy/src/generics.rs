fn swap<T: Copy>(a: &mut T, b: &mut T) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

// PartialOrd trait: has a function partial_cmp() -> Option<Ordering>
// Ord trait: has a function cmp() -> Ordering
// Ordering is an enum whose variants are Equal, Less, or Greater
// In PartialOrd two values may not have a defined ordering. In Ord, any two values must have a defined ordering.
fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    let mut result = list.first()?;

    for number in list {
        if number > result {
            result = number;
        }
    }

    return Some(result);
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        return &self.x;
    }

    fn mixup<X, Y>(self, other: Point<X, Y>) -> Point<T, Y> {
        return Point {
            x: self.x,
            y: other.y
        };
    }
}

impl Point<f32, f32> {
    fn norm(&self) -> f32 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt();
    }
}

fn main() {
    let mut a = 'a';
    let mut b = 'b';
    println!("a is {a} and b is {b}");
    swap(&mut a, &mut b);
    println!("a is {a} and b is {b}");
    (a, b) = (b, a);
    println!("a is {a} and b is {b}");
    // a is a and b is b
    // a is b and b is a
    // a is a and b is b

    let mut a = "aaaaa";
    let mut b = "bbbbb";
    println!("a is {a} and b is {b}");
    swap(&mut a, &mut b);
    println!("a is {a} and b is {b}");
    (a, b) = (b, a);
    println!("a is {a} and b is {b}");
    // a is aaaaa and b is bbbbb
    // a is bbbbb and b is aaaaa
    // a is aaaaa and b is bbbbb


    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let number_max = largest(&number_list);
    println!("The largest in {number_list:?} is {number_max:?}");
    // The largest in [102, 34, 6000, 89, 54, 2, 43, 8] is Some(6000)


    let char_list = vec!['d', 'g', 'a', 'f', 'x', 'c', 'e', 'b'];
    let char_max = largest(&char_list);
    println!("The largest in {char_list:?} is {char_max:?}");
    // The largest in ['d', 'g', 'a', 'f', 'x', 'c', 'e', 'b'] is Some('x')


    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    println!("Both integer: {both_integer:?}");
    println!("Both float: {both_float:?}");
    println!("Integer and float: {integer_and_float:?}");
    // Both integer: Point { x: 5, y: 10 }
    // Both float: Point { x: 1.0, y: 4.0 }
    // Integer and float: Point { x: 5, y: 4.0 }
}
