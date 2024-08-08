fn main() {    
    // Stack and Heap, Pointer and dynamic sized data
    // fn a() {
    //     let x:&str = "hello"; // string literal, stored in binary on stack
    //     let y:i32 = 22; // stack
    //     b();
    // }
    // fn b() {
    //     let x:String = String::from("world"); // dynamic sized, pointer stored in stack, data stored in heap
    // }

    // Ownership rules
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    { // s is not valid here, it’s not yet declared
        let s: String = String::from("hello"); // s is valid from this point forward
        // do stuff with s
    } // this scope is now over, and rust drops the value, s is no longer valid


    // Copy, Move, and Clone
    // Copy: stack only, simple types, no heap data, no Drop trait
    let x:i32 = 5;
    let y:i32 = x; // x is copied to y, both are valid

    let s1 = String::from("hello");
    //let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    let s2 = s1.clone(); // s1 is cloned to s2, both are valid
    
    println!("{}", s1); 


    // Ownership and Functions
    let s = String::from("hello");
    takes_ownership(s); // s is moved to the function, s is no longer valid
    // println!("{}", s); // error: value borrowed here after move

    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    }

    let x:i32 = 5;
    makes_copy(x); // x is copied to the function, x is still valid
    println!("{}", x);

    fn makes_copy(some_integer: i32) {
        println!("{}", some_integer);
    }

    // Return Values and Scope
    let s1: String = gives_ownership(); // some_string is moved to s1
    let s2: String = String::from("hello");
    let s3: String = takes_and_gives_back(s2); // s2 is moved to the function, s2 is no longer valid
    println!("s1: {}, s3: {}", s1, s3);

    // References and Borrowing
    // References '&' allow you to refer to some value without taking ownership of it
    // you can have multiple immutable references to a piece of data
    let s1: String = String::from("hello");
    let len: usize = calculate_length(&s1); // s1 is borrowed, don't take the ownership, s1 is still valid
    println!("The length of '{}' is {}.", s1, len);

    // Mutable References
    // there can be only one mutable reference to a particular piece of data in a particular scope
    // you cannot have a mutable reference while an immutable one is active

    let mut s1: String = String::from("hello");
    change(&mut s1); // s1 is borrowed as mutable, don't take the ownership, s1 is still valid

    // Dangling References
    let s:String = String::from("hello");


    // The Rules of References
    // 1. At any given time, you can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid.


    // Slices
    // slices won't take ownership, they are references to a part of a String
    let mut s: String = String::from("hello world");
    // let hello: &str = &s[0..5]; // &s[..5]
    // let world: &str = &s[6..11]; // &s[6..]
    let s2: &str = "hello world"; // string literals are slices, stored in binary on stack 

    let word:&str = first_word(&s);
    s.clear(); // error: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("{}", word); // error: value borrowed here after move


    // array slices
    let a:[i32; 5] = [1, 2, 3, 4, 5];
    let slice:&[i32] = &a[0..2];


}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    let length: usize = s.len(); // len() returns the length of a String
    length
}

fn change(s: &mut String) {
    s.push_str(", world"); // push_str() appends a literal to a String
    println!("{}", s);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..] // return the whole string 
}