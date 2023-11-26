// All iterators implement a trait from the standard library named Iterator:
/*
pub trait Iterator {
    type Item; // <-- Associated type

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
 */


 fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }
    // Got: 1
    // Got: 2
    // Got: 3

    let mut v1_iter = v1.iter();
    println!("I'M GONNA PRINT FOUR ELEMENTS!");
    println!("Next: {:?}", v1_iter.next());
    println!("Next: {:?}", v1_iter.next());
    println!("Next: {:?}", v1_iter.next());
    println!("Next: {:?}", v1_iter.next());
    // I'M GONNA PRINT FOUR ELEMENTS!
    // Next: Some(1)
    // Next: Some(2)
    // Next: Some(3)
    // Next: None

    // Note that each `.next()` returns an `Optional` with a reference to the element within the collection.
    // If we wanted to return the element itself, and thus take ownership of it, we can use `.into_iter()`.

    // Examples of what type returns each iterator:
    // let a: &i32 = v1.iter().next().unwrap(); // <-- Reference
    // let a: &mut i32 = v1.iter_mut().next().unwrap(); // <-- Mutable reference
    // let a: i32 = v1.into_iter().next().unwrap(); // <-- The value itself, which you now own.

    // All iterators must implement the `.next()` method, but the `Iterator` trait also has a bunch of other
    // methods with default implementations. Methods that call `.next()` behind the scenes are called
    // _consuming adaptors_. An example is `.next_chunk()`, `.last()`, `.nth()`, `.sum()`, etc. Note that
    // `.sum()` takes ownership of the iterator, so that iterator cannot be used again afterwards!

    let mut v1_iter = v1.iter();
    let second_element = v1_iter.nth(1);
    let sum_of_remaining: i32 = v1_iter.sum();
    println!("The second element is {second_element:?} and the sum of the remaining elements is {sum_of_remaining}");
    // The second element is Some(2) and the sum of the remaining elements is 3

    let sum_total: i32 = v1.iter().sum();
    println!("The sum of all elements is {sum_total}");
    // The sum of all elements is 6

    // Some iterator methods produce other iterators. These are called _Iterator Adaptors_. Note that these
    // methods take ownership of the iterator! Some of the most common ones are `.filter()` and `.map()`:
    for ele in v1.iter().map(|x| x + 1) {
        println!("Got: {ele}");
    }
    // Got: 2
    // Got: 3
    // Got: 4

    // What if we want to turn the result of this adapted iterator into a Vec? We can use `.collect()`, a
    // method that creates a new collection with the elements of the iterator!
    let v2: Vec<_> = v1.iter().filter(|x| *x % 2 == 1).map(|x| x + 1).collect();
    println!("v1 is {v1:?} and v2 is {v2:?}");
    // v1 is [1, 2, 3] and v2 is [2, 4]
}
