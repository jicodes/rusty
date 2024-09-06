


fn main() {
    // type aliases
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);


    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        // --snip--
        Box::new(|| println!("hi"))
    }


    // never type
    while game_in_progress() {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }

    // loop never returns
    loop {
        println!("This prints forever!");
    }

    // dynamic sized types
    let s1: &str = "Hello, world!";
    let s2: &str = "Hello, world!";
    
}

// rust add sized trait to generic functions
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}

impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"), // panic! macro returns ! (a never type)
        }
    }
}

// The Never Type that never returns
fn bar() -> ! {
    panic!("This function never returns!");
}