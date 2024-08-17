// struct cannot live longer than the reference it holds
struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl <'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) -> &str {
        println!("attention please: {}", announcement);
        self.part
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {

    // lifetime annotation in struct
    // let novel = String::from("Call me Ishmael. Some years ago...");
    // let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    // let i = ImportantExcerpt { part: first_sentence };


    // dangling reference
    // let r: &i32;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r);

    // lifetime annotation
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);


    // static lifetime
    // all string literals have 'static lifetime
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);

}

// &i32       // a reference
// &'a i32    // a reference with an explicit lifetime
// &'a mut i32// a mutable reference with an explicit lifetime

// generic lifetime annotation
// it doesn't change the lifetime of any value passed in, but specifies the relationship between the lifetimes of multiple references
// the lifetime of return value will be the same as the smallest lifetimes of the references passed in
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} 

// lifetime elision rules
// 1. each parameter that is a reference gets its own lifetime parameter
// 2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
// 3. if there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters
fn first_word(s: &str) -> &str { 
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

