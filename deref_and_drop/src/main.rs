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

// Drop trait
struct CustomSmartPointer {
    data: String,
}

// In Rust, the Drop trait is automatically in scope
// because it’s part of the standard prelude
// impl Drop for CustomSmartPointer with `drop` method
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
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

    // Drop trait

    // the data will be dropped in reverse order of their creation
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    drop(c); // provided by the standard library, explicitly drop a value, change the order
    println!("CustomSmartPointers created.");
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
