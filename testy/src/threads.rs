use std::thread;
use std::time::Duration;

// Note: When the main thread completes, all remaining spawned threads are shut down.

fn main() {
    println!("Thread that is running main: {:?}", thread::current().id());
    // Thread that is running main: ThreadId(1)

    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("Hi number {i} from thread {:?}!", thread::current().id());
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::sleep(Duration::from_millis(50));
    for i in 0..5 {
        println!("Hi number {i} from thread {:?}!", thread::current().id());
        thread::sleep(Duration::from_millis(100));
    }

    // The prints happen interspersed (this is most likely, but not guaranteed):
    // Hi number 0 from thread ThreadId(2)!
    // Hi number 0 from thread ThreadId(1)!
    // Hi number 1 from thread ThreadId(2)!
    // Hi number 1 from thread ThreadId(1)!
    // Hi number 2 from thread ThreadId(2)!
    // Hi number 2 from thread ThreadId(1)!
    // Hi number 3 from thread ThreadId(2)!
    // Hi number 3 from thread ThreadId(1)!
    // Hi number 4 from thread ThreadId(2)!
    // Hi number 4 from thread ThreadId(1)!

    handle.join().expect("Couldn't join the thread :(");

    // This code does not compile:
    /* let v = vec![1, 2, 3];
    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    }); */
    // The reason is that thread::spawn takes in a `'static` closure, and unlike before, this
    // closure is not static, as it only lives as long as the vector it is borrowing. We can
    // fix this by _moving_ the vector into the closure, so ownership of `v` is transferred
    // to the new thread:

    let mut my_string = String::from("some random ass _COSA IMPORTANTE_");

    let handle = thread::spawn(move || {
        my_string.clear();
        my_string.push_str("I TOOK YOUR STRING!");
        println!("Haha! The string is mine! {my_string}");
    });

    handle.join().unwrap();
    // Haha! The string is mine! I TOOK YOUR STRING!
}
