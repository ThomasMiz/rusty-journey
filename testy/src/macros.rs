// _Macros_ are a family of features in Rust that include _declarative macros, done with
// `macro_rules!`, and three kinds of _procedural macros_ that include:
// - custom `#[derive]` macros
// - Attribute-like macros, usable on any items
// - Function-like macros, that look like function calls but operate on the tokens specified
//       as their argument.

// Declarative macros take in, as arguments, literal Rust source code, and patterns are
// matched against those to write the resulting Rust code that replaces the macro call.

// Here is an example of a very simplified version of the `vec!` macro:
#[macro_export]
macro_rules! myvec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// Note: The `#[macro_export]` is used to make the macro publicly visible from this module.

// Inside the macro, we have a set of patterns, followed by `=>`, followed by a block of code
// associated with said pattern. In the case of `myvec!`, we only have one pattern.

// Macros by example: https://doc.rust-lang.org/reference/macros-by-example.html
// The Little Book of Rust Macros: https://veykril.github.io/tlborm/

// Procedural macros act more like functions. They accept some code as input, operate on that
// code, and produce some code as an output. We define them as functions that take a
// `TokenStream` as input and return a `TokenStream` as output. This type is defined in the
// `proc_macro` crate.

fn main() {
    let v: Vec<u32> = myvec![1, 2, 3];
    println!("{v:?}");
    // [1, 2, 3]
}
