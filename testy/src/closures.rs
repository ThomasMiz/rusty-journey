/*
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
 */


/*
There are three Fn traits that closures can progressively implement, and which of these they implement
depends on which variables they capture from their environment, and how (does it borrow? mutably or
immutably? or does it take ownership?).

There are the traits:
- `FnOnce`: Applies to closures that can be called (at least) once. All closures implement this trait.
    A closure that moves captured values out of its body can only be called once, so it implements this
    but not any of the other traits.
- `FnMut`: Applies to closures that don't move captured values out of their body, but that might mutate
    captured values. These closures can be called more than once.
- `Fn`: Applies to closures that don't move nor mutate any captured values. This includes, for example,
    closures that don't capture anything.

For example, look at the definition of Option<T> .unwrap_or_else:
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T // `F` must be able to be called once, take no arguments, and return a T.
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
 */

 use std::thread;

 #[derive(Debug, PartialEq, Copy, Clone)]
 enum ShirtColor {
     Red,
     Blue,
 }
 
 struct Inventory {
     shirts: Vec<ShirtColor>,
 }
 
 impl Inventory {
     fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
         user_preference.unwrap_or_else(|| self.most_stocked())
     }
 
     fn most_stocked(&self) -> ShirtColor {
         let mut num_red = 0;
         let mut num_blue = 0;
 
         for color in &self.shirts {
             match color {
                 ShirtColor::Red => num_red += 1,
                 ShirtColor::Blue => num_blue += 1,
             }
         }
         if num_red > num_blue {
             ShirtColor::Red
         } else {
             ShirtColor::Blue
         }
     }
 }
 
 #[derive(Debug)]
 struct Rectangle {
     width: u32,
     height: u32,
 }
 
 fn main() {
     let store = Inventory {
         shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
     };
 
     let user_pref1 = Some(ShirtColor::Red);
     let giveaway1 = store.giveaway(user_pref1);
     println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);
     // The user with preference Some(Red) gets Red
 
     let user_pref2 = None;
     let giveaway2 = store.giveaway(user_pref2);
     println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);
     // The user with preference None gets Blue
 
     let mut list = vec![1, 2, 3];
     println!("Before defining closure: {:?}", list);
     // Before defining closure: [1, 2, 3]
 
     let mut borrows_mutably = || list.push(7);
     // Attempting immutably borrow list here will fail. `println!("{list:?}");` throws an error!
     borrows_mutably();
     println!("After calling closure: {:?}", list);
     // After calling closure: [1, 2, 3, 7]
 
     let takes_ownership = move || println!("The list is {list:?}");
     // Attempting to use list from here on throws an error- it has been moved into the closure.
     takes_ownership();
     // The list is [1, 2, 3, 7]
 
     // This is useful when passing things to threads, now the list is the thread's property!
     let list = vec![1, 2, 3];
     thread::spawn(move || println!("New thread prints the list! {list:?}"))
         .join()
         .unwrap();
     // New thread prints the list! [1, 2, 3]
 
 
 
     let mut list = [
         Rectangle { width: 10, height: 1 },
         Rectangle { width: 3, height: 5 },
         Rectangle { width: 7, height: 12 },
     ];
 
     list.iter().for_each(|r| println!("Another rectangle! {} {}", r.width, r.height));
     // Another rectangle! 10 1
     // Another rectangle! 3 5
     // Another rectangle! 7 12
 
     // `.sort_by_key` can't ask for an `FnOnce`, because it needs to call that closure multiple times,
     // so it asks for an `FnMut` instead, which can be called multiple times. One side effect of this
     // is that we can modify captured variables! So we can, for example, count the number of calls:
     let mut total_calls = 0;
     list.sort_by_key(|r| { total_calls += 1; r.width });
     println!("{:#?}", list);
     println!("Total calls: {total_calls}");
     // [
     //     Rectangle { width: 3, height: 5, },
     //     Rectangle { width: 7, height: 12, },
     //     Rectangle { width: 10, height: 1, },
     // ]
     // Total calls: 6
 }
 