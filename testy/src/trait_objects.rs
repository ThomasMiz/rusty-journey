// Rust doesn't have inheritance like typical OOP languages do, but there are other
// tools we can use to easily get our program to do what we want.

// In this example, we'll implement a simple GUI, where we handle a list of UI
// elements and want to be able to draw them all to the screen. The caveat is that
// we want the library user to be able to make their own GUI items, which therefore
// are not defined in our library!

// In an OOP paradigm, we'd make a base class `Component` and define an abstract
// `draw()` method for subclasses to override. In Rust, we'll use _trait objects_.

// Trait objects points to both an instance of a type implementing our specified
// trait, and a table used to look up trait methods on that type at runtime. We
// use trait objects by using some sort of pointer, like `&` or `Box<T>`, with the
// `dyn` keyword, followed by the desired trait.

// Note: Prefer using simple traits bounds, like `Screen<T: Draw>`, because even
// though these are more restrictive, these are monomorphized at compile time and
// are more performant, as method calls don't need to go through a lookup table.

// This is the difference between _static dispatch_ and _dynamic dispatch_. With
// static dispatch, the compiler knows at compile time what function you're
// calling and can simply call it. With dynamic dispatch, the compiler doesn't
// know, so it instead it emits code to figure it out at compile time.

use crate::{gui_library::{Screen, Button}, gui_lib_extensions::Label};

mod gui_library {
    pub trait Draw {
        fn draw(&self) -> String;
    }

    // If we had used `Screen<T: Draw>`, then all components in the screen would
    // have had to be the same type. Either all `Button`, or all `TextBox`, etc.
    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn render(&self) {
            struct RenderInfo {
                width: u32,
                height: u32,
                string: String
            }

            let renders: Vec<_> = self.components.iter().map(|x| {
                let s = x.draw();
                RenderInfo {
                    width: s.lines().map(|x| x.len() as u32).max().unwrap_or(0),
                    height: s.lines().count() as u32,
                    string: s,
                }
            }).collect();

            let total_height = renders.iter().map(|x| x.height).max().unwrap_or(0);

            let mut line_iters: Vec<_> = renders.iter().map(|s| s.string.lines()).collect();

            for _ in 0..total_height {
                for i in 0..(renders.len()) {
                    let current_element = &renders[i];
                    match line_iters[i].next() {
                        Some(line) => {
                            print!("{}", line);
                            for _ in 0..(current_element.width - line.len() as u32) {
                                print!(" ");
                            }
                        },
                        None => {
                            for _ in 0..(current_element.width) {
                                print!(" ");
                            }
                        },
                    }
                }
                println!();
            }
        }
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String
    }

    impl Draw for Button {
        fn draw(&self) -> String {
            let max_line_width = self.label.lines().map(|x| x.len() as u32).max().unwrap_or(0);
            let actual_width = u32::max(self.width, max_line_width + 2);
            let line_count = self.label.chars().filter(|x| x.is_whitespace()).count() as u32;
            let actual_height = u32::max(self.height, line_count + 2);
            let empty_line_count = actual_height - 2 - line_count;

            let mut result = String::new();

            for ele in 0..actual_width {
                result.push(if ele == 0 || ele == actual_width - 1 {'+'} else {'-'} );
            }
            result.push('\n');

            if empty_line_count > 1 {
                for _ in 0..(empty_line_count / 2) {
                    result.push('|');
                    for _ in 0..(actual_width - 2) {
                        result.push(' ');
                    }
                    result.push('|');
                }
                result.push('\n');
            }

            for line in self.label.lines() {
                let remaining_space = actual_width - 2 - line.len() as u32;
                result.push('|');
                for _ in 0..(remaining_space / 2) {
                    result.push(' ');
                }
                result.push_str(line);
                for _ in 0..((remaining_space + 1) / 2) {
                    result.push(' ');
                }
                result.push('|');
                result.push('\n');
            }

            if empty_line_count != 0 {
                for _ in 0..((empty_line_count + 1) / 2) {
                    result.push('|');
                    for _ in 0..(actual_width - 2) {
                        result.push(' ');
                    }
                    result.push('|');
                }
                result.push('\n');
            }

            for ele in 0..actual_width {
                result.push(if ele == 0 || ele == actual_width - 1 {'+'} else {'-'} );
            }

            return result;
        }
    }
}

mod gui_lib_extensions {
    use super::gui_library;
    pub struct Label {
        pub label: String
    }

    impl gui_library::Draw for Label {
        fn draw(&self) -> String {
            let actual_width = self.label.lines().map(|x| x.len() as u32).max().unwrap_or(0);

            let mut result = String::new();
            let mut is_first = true;

            for line in self.label.lines() {
                if is_first {
                    is_first = false;
                } else {
                    result.push('\n');
                }

                let remaining_space = actual_width - line.len() as u32;
                for _ in 0..(remaining_space / 2) {
                    result.push(' ');
                }
                result.push_str(line);
                for _ in 0..((remaining_space + 1) / 2) {
                    result.push(' ');
                }
            }

            return result;
        }
    }
}

fn main() {
    let scr = Screen {
        components: vec![
            Box::new(Button {
                width: 10,
                height: 3,
                label: String::from("olus:)"),
            }),
            Box::new(Label {
                label: String::from("\nPedro...\nPedro?\n Sip. Pedro =D "),
            }),
            Box::new(Button {
                width: 1,
                height: 1,
                label: String::from("A ver flaco no\nentendes nada\nvos"),
            }),
        ],
    };

    scr.render();
    // +--------+               +--------------+
    // | olus:) |   Pedro...    |A ver flaco no|
    // |        |    Pedro?     |entendes nada |
    // +--------+ Sip. Pedro =D |     vos      |
    //                          +--------------+
}
