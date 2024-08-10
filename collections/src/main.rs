use unicode_segmentation::UnicodeSegmentation;

use std::collections::HashMap;

fn main() {
    // Vectors
    let a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", a);

    let third: &i32 = &v[2];
    // v.push(4); error: mutable borrow occurs here, but 
    println!("The third element is {}", third); // immutable borrow later used here

    // get method of vector returns an Option<&T>
    match v.get(20) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &mut v {
        *i += 50; // dereference i to change the value it refers to
    }

    for i in &v {
        println!("{}", i);
    }


    {
        let _v2 = vec![1, 2, 3];
    }

    // store different types in a vector via an enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Strings: stored as a collection of UTF-8 encoded bytes
    let mut s1 = String::new();
    let s2: &str = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");

    // appending to a string
    s1.push_str("foo");
    let s5 = String::from("ba");
    let mut s6: String = s1 + &s5; // s1 has been moved here and can no longer be used
    
    s6.push('r');
    s6.push('!');
    println!("{}", s6);

    // format! macro won't take ownership of any of its parameters
    let mut s7 = format!("{}-{}", s4, s6); 
    println!("{}", s7);

    // Indexing into strings
    let hello = String::from("नमस्ते");
    // let c:char = hello[0]; // error: cannot index into a string in rust
    
    // bytes() method returns a sequence of bytes
    for byte in hello.bytes() {
        println!("{}", byte);
    }

    // chars() method returns a sequence of chars
    for c in hello.chars() {
        println!("{}", c);
    }

    // grapheme clusters: a single visual character that users would recognize as a single character
    // unicode-segmentation crate can be used to iterate over grapheme clusters
    for g in hello.graphemes(true) {
        println!("{}", g);
    }

    // Hash Maps
    let blue = String::from("blue");
    let yellow = String::from("yellow");
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("The score of the blue team is {}", s),
        None => println!("The blue team has no score"),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // updating a value in a hash map
    scores.insert(String::from("blue"), 25);
    // won't update the value if the key exists
    scores.entry(String::from("green")).or_insert(50);

    // updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map: HashMap<&str, i32> = HashMap::new();

    // ["hello", "world", "wonderful", "world"]
    for word in text.split_whitespace() {
        let count:&mut i32 = map.entry(word).or_insert(0); // returns a mutable reference to the value
        *count += 1;
    }

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
    
    
}
