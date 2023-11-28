// Smart pointers are data structures that act like pointers, but store additional metadata
// and provide more capabilities (at a cost). All smart pointers own the data they point to.

// Smart pointers are implemented as structs that `impl` the `Deref` and `Drop` traits:
// - `Deref`: Allows using an instance of a smart pointer as if it were a simple reference.
// - `Drop`: Allows you to customize the code that's run when the pointer goes out of scope.

// We won't cover all smart pointers in this chapter. You can also write your own, and some
// libraries do have their own smart pointers. We will cover:
// - `Box<T>` for allocating values on the heap
// - `Rc<T>` a reference counting type that enables multiple ownership
// - `Ref<T>`, `RefMut<T>`, accessed through `RefCell<T>`, enforces the borrowing rules at
//       runtime instead of compile time.

use std::{ops::Deref, rc::{Rc, Weak}, fmt::Debug, cell::RefCell};

struct StupidBox<T>(T); // A tuple struct with just one element of type T

impl<T> StupidBox<T> {
    fn new(x: T) -> StupidBox<T> {
        StupidBox(x)
    }
}

// Note: `Deref` is for dereferencing immutable references. For mutable references, use
// `DerefMut` instead.
impl<T> Deref for StupidBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

// Another trait at our disposal is `Drop`. It defines a `drop` function that gets called
// when the resource goes out of scope.
struct ShittyString {
    data: String
}

impl Drop for ShittyString {
    fn drop(&mut self) {
        println!("Dropping ShittyString with data {}", self.data);
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    // `Box<T>` is most often useful when:
    // - You have a type whose size can't be known at compile time
    // - You have a large amount of data and you want to transfer ownership of it but
    //       you want to ensure it's not getting copied around when you do.
    // - When you want to own a value that you only care implements a particular trait,
    //       rather than being of a specific type.

    let b = Box::new(5);
    println!("b = {b}");
    // b = 5

    // Boxes allow creating recursive types:
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil,
    }

    // A linked list!
    let my_list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    let mut current_node = &my_list;
    loop {
        match current_node {
            List::Cons(ele, next) => {
                println!("Next element: {ele}");
                current_node = next;
            },
            List::Nil => break,
        }
    }
    // Next element: 1
    // Next element: 2
    // Next element: 3


    // `Box<T>` implements the `Deref` trait, so it can be used as if it were a reference
    let x = 5;
    let y1 = &x;
    let y2 = Box::new(x);
    assert_eq!(x, *y1);
    assert_eq!(x, *y2);


    // Our `StupidBox<T>` also implements the `Deref` trait, so the same thing applies!
    let x = 5;
    let y1 = &x;
    let y2 = StupidBox::new(x);
    assert_eq!(x, *y1);
    assert_eq!(x, *y2); // Behind the scenes, this is doing *(y2.deref())


    // _Deref coercion_ converts a reference to a type into a reference of another type, by
    // having it implement the `Deref` trait. For example, `&String` can be converted into
    // a `&str`, because `String` implements `Deref` for converting into a `&str`.
    let m = StupidBox::new(String::from("Pedro"));
    hello(&m); // `StupidBox<String>` is deref-ed into `&String` and again into `&str`
    // Hello, Pedro!

    // Note: if we wanted to do something like m.push_str("hello!"), we'd need to impl
    // `DerefMut` for `StupidBox<T>`. `Deref` returns only _immutable_ references!

    let s1 = ShittyString { data: String::from("First") };
    let s2 = ShittyString { data: String::from("Second") };
    println!("Created two ShittyStrings: {} and {}", s1.data, s2.data);

    // If we want to manually drop a value from memory, we can't call it's `.drop()` function,
    // this will throw a compile error. We can instead use `std::mem::drop()`:
    drop(s2);
    // Dropping ShittyString with data Second
    drop(s1);
    // Dropping ShittyString with data First




    // `Rc<T>` is a reference-count smart pointer. The value it points to is owned by all
    // copies of the pointer, and the value is dropped only once they are all gone.
    // This is useful when we have some data used by multiple parts of the program, but
    // there's no way to determine at compile time which part will finish last. Note that
    // `Rc<T>` is only for single-threaded scenarios!

    // Let's showcase this by implementing the following linked list structure:
    // b --> [3]------|
    //                V
    //         a --> [5]----> [10]----> [Nil]
    //                A
    // c --> [4] -----|
    // Our existing `List<T>` type can't handle this! We could make the list use references
    // instead of each node owning the next value, but then we'll need lifetime parameters,
    // and all of the list's values will have to live as long as the entire list!

    enum RcList<T> {
        Cons(T, Rc<RcList<T>>),
        Nil,
    }

    fn print_rclist<T: Debug>(node: &RcList<T>, name: &str) {
        print!("The {name} list goes:");
        let mut current = node;
        loop {
            match current {
                RcList::Cons(ele, next) => {
                    print!(" {ele:?}");
                    current = next;
                },
                RcList::Nil => break,
            }
        }
        println!();
    }

    let a = Rc::new(RcList::Cons(5, Rc::new(RcList::Cons(10, Rc::new(RcList::Nil)))));
    println!("There are {} strong references to a.", Rc::strong_count(&a));
    // There are 1 strong references to a.

    let b = RcList::Cons(3, Rc::clone(&a));
    println!("There are {} strong references to a.", Rc::strong_count(&a));
    // There are 2 strong references to a.

    let c = RcList::Cons(4, Rc::clone(&a));
    println!("There are {} strong references to a.", Rc::strong_count(&a));
    // There are 3 strong references to a.

    let d = a.clone();
    println!("There are {} strong references to a.", Rc::strong_count(&a));
    // There are 4 strong references to a.
    drop(d);
    println!("Wait no, they're back down to {}.", Rc::strong_count(&a));
    // Wait no, they're back down to 3.


    // We could have used `a.clone()` instead of `Rc::clone(&a)`, but `.clone()` is a method
    // that typically does a deep copy of the underlying data. Even though this is not the
    // case with `Rc<T>`, it's clearer to just use `Rc::clone` instead.

    print_rclist(&a, "a");
    // The a list goes: 5 10
    print_rclist(&b, "b");
    // The b list goes: 3 5 10
    print_rclist(&c, "c");
    // The c list goes: 4 5 10

    // Note that `Rc<T>` allows reading values as immutable, but not mutable. If it allowed
    // you to mutate the value it points to, you could have multiple mutable references to
    // that data, which violates the borrowing rules and thus can cause data races and
    // inconsistencies. For this, we have the _interior mutability_ design pattern.

    // _Interior mutability_ is a design pattern in Rust that allows us to mutate data even
    // when there are immutable references to that data. This is normally disallowed by the
    // borrowing rules, but this pattern uses `unsafe` code because "fuck it" (??? bruh?).

    // `unsafe` code typically manually enforces mutation and borrowing rules, and that code
    // is then wrapped in a safe API.

    // `RefCell<T>` is a type that follows the interior mutability pattern. `RefCell<T>` is
    // used to enforce the borrowing rules at runtime rather than compile time. If you break
    // the borrowing rules with `RefCell<T>`, your program will panic.

    // This exists because some programs might be memory-safe, but the Rust compiler isn't
    // able to, with its static analyzer, determine that at compile-time, and therefore it
    // would reject the program and not compile.

    // Note: `RefCell<T>` is also only for use in single-threaded scenarios.

    // (Look at the _mock_messages_ project for more on `RefCell<T>`).

    // Another use for `RefCell<T>` is to be able to make our linked list's elements mutable!

    #[derive(Debug)]
    enum RcMutList<T> {
        Cons(Rc<RefCell<T>>, Rc<RcMutList<T>>),
        Nil,
    }

    let value_a = Rc::new(RefCell::new(5));

    let a = Rc::new(RcMutList::Cons(Rc::clone(&value_a), Rc::new(RcMutList::Cons(Rc::new(RefCell::new(10)), Rc::new(RcMutList::Nil)))));
    let b = RcMutList::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = RcMutList::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("a before: {a:?}");
    println!("b before: {b:?}");
    println!("c before: {c:?}");
    // a before: Cons(RefCell { value: 5 }, Cons(RefCell { value: 10 }, Nil))
    // b before: Cons(RefCell { value: 3 }, Cons(RefCell { value: 5 }, Cons(RefCell { value: 10 }, Nil)))
    // c before: Cons(RefCell { value: 4 }, Cons(RefCell { value: 5 }, Cons(RefCell { value: 10 }, Nil)))

    *value_a.borrow_mut() += 5;

    println!("a after: {a:?}");
    println!("b after: {b:?}");
    println!("c after: {c:?}");
    // a after: Cons(RefCell { value: 10 }, Cons(RefCell { value: 10 }, Nil))
    // b after: Cons(RefCell { value: 3 }, Cons(RefCell { value: 10 }, Cons(RefCell { value: 10 }, Nil)))
    // c after: Cons(RefCell { value: 4 }, Cons(RefCell { value: 10 }, Cons(RefCell { value: 10 }, Nil))

    // As we can see, the value of node a was updated, and the change is visible on all lists!

    // If we were working with multithreaded code, none of these smart pointers would work. `Rc<T>`
    // has a thread-safe version, `Arc<T>` or Atomically Reference Counted, and `RefCell<T>` has
    // `Mutex<T>`.

    // CAREFUL!! Rc<T> and RefCell<T> combined have the ability to create a reference cycle.
    // In such cases, an Rc<T> is pointing to data that contains a second Rc<T> pointing to
    // data that contains the original Rc<T>. If you stop referencing these, since the
    // reference counters haven't reached 0, _they will not be deallocated_, causing a
    // memory leak.

    #[derive(Debug)]
    enum Maybe {
        Cons(i32, Rc<RefCell<Maybe>>),
        Nil
    }

    let a_next = Rc::new(RefCell::new(Maybe::Nil));
    let a = Rc::new(RefCell::new(Maybe::Cons(1, Rc::clone(&a_next))));
    let b = Rc::new(RefCell::new(Maybe::Cons(2, Rc::clone(&a))));

    *a_next.borrow_mut() = Maybe::Cons(1, Rc::clone(&b));

    // Interesting detail: this print here would cause a stack overflow:
    // println!("{a:?}");
    // RefCell { value: Cons(1, RefCell { value: Cons(1, RefCell { value: Cons(2, RefCell { value: Cons(1, RefCell { ...

    drop(a_next);
    drop(a);
    drop(b);
    println!("Memory has been leaked 💀");
    // Running this program and analyzing it with Valgrind, it detects a memory leak!

    // One way to avoid this is by using _weak references_. Weak references don't count
    // towards the reference count, but instead have a different counter, the _weak count_
    // (with the other counter being the _strong count_). You can create a weak reference
    // by passing an `Rc<T>` to `Rc::downgrade()`, which returns a `Weak<T>`. This type
    // contains an `Option<Rc<T>>`, which will be `Some(Rc<T>)` if the value has not been
    // dropped, or will be `None` if it has. You can get this with `weak.upgrade()`.

    // In this next example, we'll implement a tree structure where nodes own their
    // children, but also have references to their parents. We can't use a plain `Rc<T>`
    // for a reference to a node's parent, that would make a reference cycle! So instead
    // we use `Rc<T>` from the parent to its children, and `Weak<T>` for the children to
    // its parent.

    #[derive(Debug)]
    struct Node<T> {
        value: T,
        parent: RefCell<Weak<Node<T>>>,
        children: RefCell<Vec<Rc<Node<T>>>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // leaf parent = None

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) }, children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] } })
}
