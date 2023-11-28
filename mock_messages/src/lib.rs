pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// Let's say we now want to test this messanger, but instead of doing something like printing the
// strings to stdout, we want to store them so we can then check they are the correct ones. One
// would typically create a Vec<String> with all the strings received, and append to it. We then
// have a problem! Because the `Messanger` trait's `send()` method's signature specifies it
// receives self as an _immutable reference_! We can't `.push()` things into a `Vec` if it's
// immutable! Here's where `RefCell<T>` comes in handy.

// We keep everything immutable, and use `RefCell<T>` to, behind the scenes, magically get a
// mutable borrow of the vec. Because fuck the borrowing and ownership rules, rules were meant
// to be broken, and we're badass rulebreakers, and anyone who complains will get their knees
// broken too.

// Note that `RefCell<T>.borrow()` and `.borrow_mut()` return types `Ref<T>` and `RefMut<T>`
// respectively. The `RefCell<T>` keeps track of how many of these there are active, and uses
// that to enforce the borrowing rules. If we `.borrow_mut()` once, and then `.borrow_mut()`
// again without dropping the first one, our program will panic.

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        // sent_messages: Vec<String>, // Immutable, won't work in the impl block's send method!
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // sent_messages: vec![],
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // self.sent_messages.push(String::from(message)); // <-- Error! `self` is an immutable reference!
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // assert_eq!(mock_messenger.sent_messages.len(), 1);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}