// The _state pattern_ is an OO design pattern where we define a set of states a value
// can have internally. The states are represented by a set of _state objects_.

// We will make an example where a post in a blog starts as an empty "draft", then once
// ready it gets marked for review, and once it's approved, it gets published. So the
// states are "draft" --> "review" --> "published".

// Any changes to a post that are not allowed by the current state should have no
// effect. Posts will provide methods to `add_text`, `request_review`, and `approve`.

mod blog {
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    // Note: `state` will always be Some(something). We need to use an `Option<T>` or
    // something we can "temporarily invalidate", because otherwise in our `approve` or
    // `request_review` (or any method that changes the `Post`'s state) we wouldn't be
    // able to update the state, as `state.request_review()` and other such methods are
    // consuming the state variable and returning a new box. Rust doesn't allow this
    // variable to be moved out of the struct, so instead we can wrap it in an `Option`
    // and set it to `None`.

    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.state.as_ref().unwrap().add_text(&mut self.content, text);
        }

        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }

        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review());
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve());
            }
        }

        pub fn reject(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.reject());
            }
        }
    }

    trait State {
        fn add_text(&self, _content: &mut String, _text: &str) {

        }

        fn request_review(self: Box<Self>) -> Box<dyn State>;
        // Note: `self: Box<Self>` makes sure this method is only valid on a box the type
        // of the implementor of the trait. This means that `.request_review() cannot be
        // called on a `Draft`, but can be called on a `Box<Draft>`. You should also note
        // that calling `.request_review()` on a `Box<Draft>` will call the method in the
        // `impl State for Draft`, and not for `PendingReview`.

        fn reject(self: Box<Self>) -> Box<dyn State>;

        fn approve(self: Box<Self>) -> Box<dyn State>;

        fn content<'a>(&self, _post: &'a Post) -> &'a str {
            ""
        }
    }

    // Our state objects are `Draft`, `PendingReview`, and `Published`.
    struct Draft {}

    impl State for Draft {
        fn add_text(&self, content: &mut String, text: &str) {
            content.push_str(text);
        }

        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview { total_approves: 0 })
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    struct PendingReview {
        total_approves: u32,
    }

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft {})
        }

        fn approve(mut self: Box<Self>) -> Box<dyn State> {
            let state = self.as_mut();
            state.total_approves += 1;
            if state.total_approves < 2 {
                self
            } else {
                Box::new(Published {})
            }
        }
    }

    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }
}

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    post.add_text("pero la PUCHA");
    assert_eq!("I ate a salad for lunch today", post.content());
}

// This technique allows us to implement an object oriented state pattern, but we're not
// taking advantage of many of Rust's strenghts. For example, our state transitions are
// very coupled; if we wanted to add a new `Scheduled` state between `PendingReview` and
// `Published`, we'd have to add code to `PendingReview` to switch to `Scheduled`.

// We also have a lot of duplicated logic. The methods on `Post` are very similar, and
// each state struct's impl block has to implement many repeated methods.

// A better solution might be to take advantage of Rust's strong type system and create
// different structs for each post state, as in, `Post`, `DraftPost`, etc. This means
// that we can simply not have a `.content()` method for `DraftPost`, so instead of
// specifying that it should only be called on published posts, we can simply _not have
// it_ where it should't be called.

// Transitions between states would then be implemented as transformations into different
// types!

/*
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
*/

// Note how methods that cause state transitions take ownership of `self` and return a
// new struct with the moved contents, thus the old state is consumed to form the new.

// With these changes however, we're no longer following the OO state pattern anymore, as
// the transformations between states are no longer encapsulated entirely within the same
// type.
