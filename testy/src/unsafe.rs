// Within an `unsafe` block, we have these _unsafe superpowers_:
// - Dereference a raw pointer
// - Call an unsafe function or method
// - Access or modify a mutable static variable
// - Implement an unsafe trait
// - Access fields of `union`s

// Unsafe doesn't disable the borrow checker or other safety features, if you use a
// reference in unsafe code it'll still be checked. Unsafe simply gives you access
// to those features.

fn main() {
    // Dereferencing a raw pointer:
        // Raw pointers are written as `*const T` and `*mut T`. These can ignore the
        // borrowing rules, aren't guaranteed to point to valid memory, may be null,
        // and don't have automatic cleanup.

    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    // Note: We don't need `unsafe` to create raw pointers, we just can't dereference
    // them without `unsafe`. Also note that r1 and r2 point to the same memory! Even
    // though one is mutable and the other immutable!

    let address = 0x012345usize;
    let r = address as *const i32;
    // Create a pointer from a numeric memory address

    unsafe {
        println!("r1 is {} and r2 is {}", *r1, *r2);
        // r1 is 5 and r2 is 5

        // println!("PANIC TIMEEE :D {}", *r);
    }

    // Functions can be `unsafe` too, which allows using unsafe code within them,
    // but such functions can only be called from other unsafe code.

    unsafe fn dangerous() {
        println!("oooooo :O");
    }

    unsafe {
        dangerous();
        // oooooo :O
    }

    // An example of unsafe is the `split_at_mut` method from the standard library:
/*
    pub fn split_at_mut(&mut self, mid: usize) -> (&mut str, &mut str) {
        // is_char_boundary checks that the index is in [0, .len()]
        if self.is_char_boundary(mid) {
            let len = self.len();
            let ptr = self.as_mut_ptr();
            // SAFETY: just checked that `mid` is on a char boundary.
            unsafe {
                (
                    from_utf8_unchecked_mut(slice::from_raw_parts_mut(ptr, mid)),
                    from_utf8_unchecked_mut(slice::from_raw_parts_mut(ptr.add(mid), len - mid)),
                )
            }
        } else {
            slice_error_fail(self, 0, mid)
        }
    }
*/
    // Note that this function must take _two mutable borrows_ from a slice. This
    // is normally not allowed. However, since we're borrowing two non-overlapping
    // slices, it shouldn't really be an issue.

    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                std::slice::from_raw_parts_mut(ptr, mid),
                std::slice::from_raw_parts_mut(ptr.add(mid), len - mid)
            )
        }
    }

    let mut my_vec = [1, 2, 3, 4, 5];
    let a = &mut my_vec;
    let (b, c) = split_at_mut(a, 2);
    println!("b={b:?} ; c={c:?}");
    // b=[1, 2] ; c=[3, 4, 5]

    main2();
}

// The `extern` function allows us to interact with code written in other languages.
// This is an FFI, or _Foreign Language Interface_. Note that these functions are
// always unsafe to use, as Rust can't ensure any guarantees about them.

// The `extern` keyword is followed by the ABI the external function uses. This
// specifies how the function is called at the assembly level.
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main2() {
    unsafe {
        let value = 3;
        println!("The absolute value of {value}, according to C, is {}", abs(value));
        // The absolute value of 3, according to C, is 3
    }

    main3();
}

// We can also expose Rust functions so they can be called from other languages!
#[no_mangle] // <-- Tells the compiler not to change the function's name.
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}



// Mutable static variables can only be accessed from unsafe code, as this may cause
// data races between threads. Immutable ones are safe though.

static HELLO_WORLD : &str = "Hello, World!"; // <-- Note: 'static by default

static mut COUNTER: u32 = 0;

fn main3() {
    println!("Hello world says: {}", HELLO_WORLD);
    // Hello world says: Hello, World!

    unsafe {
        println!("Counter is {}", COUNTER);
        // Counter is 0
        COUNTER += 1;
        println!("Counter is now {}", COUNTER);
        // Counter is now 1
    }
}

// Unsafe traits are traits that have at least one unsafe method.
unsafe trait Foo {
    fn to_fucken_ptr(&mut self) -> *mut i32;
}

unsafe impl Foo for i32 {
    fn to_fucken_ptr(&mut self) -> *mut i32 {
        return self as *mut i32;
    }
}

// Remember the `Send` and `Sync` traits? The compiler implements those automatically
// on our types composed of only `Send` and `Sync` types. However, we can also impl
// these traits manually on our types, which is unsafe, and we must uphold their
// respective guarantees manually.