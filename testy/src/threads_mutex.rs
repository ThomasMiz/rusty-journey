// A Mutex<T> can be used to enforce _mutual exclusion_ in access to some data.
use std::{sync::{Mutex, Arc}, thread};

fn main() {
    let m = Mutex::new(5);

    // The type system ensures we can't access the mutex's data without a `.lock()`, and
    // the ownership system ensures the mutex is released afterwards.
    let mut num = m.lock().unwrap();
    *num += 5;
    drop(num);

    println!("The mutex's is now: {:?}", m);
    // The mutex's is now: Mutex { data: 10, poisoned: false, .. }

    drop(m);

    // `m.lock()` returns a `MutexGuard`, a smart pointer, that lets us access the data.

    // If we want to pass a `Mutex` to a thread we have a problem, as the mutex will be
    // _moved_ into the closure. We can't solve this by `.clone()`ing like before, as
    // `Mutex` doesn't have a `.clone()` method.

    // If we wrap the mutex in an `Rc<T>`, as in `Rc<Mutex<T>>`, we'll stumble upon another
    // issue; the compiler will complain that `Rc<T>` cannot be moved between threads safely!
    // This is because `Rc<T>` does not implement the `Send` trait. This trait is used to
    // ensure that the types we use with threads are meant for use in concurrent situations.

    // `Arc<T>` is a thread-safe version of `Rc<T>`, which can be used in concurrent
    // situations. Arc stands for Atomically Reference Counted. Rust provides many built-in
    // atomic versions of the primitive types in the `std::sync::atomic` module.

    let counter = Arc::new(Mutex::new(0));
    // (Note: for a shared integer, this could have been better done with `AtomicI32`)

    let handles: Vec<_> = (0..100).map(|_| {
        let counter2 = Arc::clone(&counter);
        return thread::spawn(move || {
            let mut num = counter2.lock().unwrap();
            *num += 1;
        });
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Results: {}", *counter.lock().unwrap());
    // Results: 100

    // Note that, in the threads, even though `counter2` is an immutable variable, we
    // can get a mutable pointer from it. In other words, `Mutex<T>` also follows the
    // _interior mutability_ design pattern, just like `RefCell<T>`.

    // Also note that Rust cannot prevent you from using mutexes wrong, things like
    // deadlocks are still possible.



    // There are two marker traits that represent key concepts in concurrency: these
    // are `Sync` and `Send`. Types can implement these to mean different things:

    // `Send` indicates that ownership of values of values of said type can be
    // transferred between threads. Most types are `Send`, but some like `Rc` aren't.
    // Types that are composed entirely of `Send` types is automatically `Send` too.

    // `Sync` indicates that the type can be referenced from multiple threads. In
    // other words, `T` is `Sync` if `&T` is `Send`. Similarly, types that are
    // composed entirely of `Sync` types are also automatically `Sync` too.

    // Note: Implementing these traits manually has to be done with unsafe code.
}