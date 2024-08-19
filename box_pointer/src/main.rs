enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // Box<T> is a smart pointer because it implements the Deref trait, it stores its data on the heap,
    let b = Box::new(5);
    // b is a smart pointer,
    // which is a pointer with additional metadata and capabilities.
    println!("b = {}", b);

    // Build cons list with enum and Box<T>
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
