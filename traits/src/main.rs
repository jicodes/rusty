use std::fmt::{Display, Debug};

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    fn summarize(&self) -> String{
        format!("(Read more from {}...)", self.summarize_author()) // default implementation
    }

    fn summarize_author(&self) -> String; 
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

// traits as parameters
// can be used to accept any type that implements the Summary trait
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// syntax sugar for the above
// traits as parameters with trait bounds
fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// takes multiple parameters of the same type
fn notify3(item1: &impl Summary, item2: &impl Summary){
    //...
}

// syntax sugar for the above
fn notify4<T: Summary>(item1: &T, item2: &T){
    //...
}

// specify multiple trait bounds with the + syntax
fn notify5(item1: &(impl Summary + Display), item2: &impl Summary){
    //...
}

// syntax sugar for the above
fn notify6<T: Summary + Display>(item1: &T, item2: &T){
    //...
}

// 
fn some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) -> i32 {
    42
}


// using where clause to clean up the code
fn some_function2<T, U>(_t: &T, _u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    42
}

// returning types that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}


fn main() {

    println!("{}", returns_summarizable().summarize());
    
}
