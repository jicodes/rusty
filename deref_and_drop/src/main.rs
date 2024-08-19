use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implementing Deref trait for MyBox
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    // let y = &x;
    let y = MyBox::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y); // *y is equivalent to *(y.deref())


    // Deref coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // hello(&(*m));
    // &MyBox<String> -> &String -> &str

    // DerefMut trait for mutable references
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
