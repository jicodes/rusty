// function pointer
fn add_one(x: i32) -> i32 {
    x + 1
}

// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//     f(arg) + f(arg)
// }

// closure traits:
// Fn: the closure captures by reference (&T) immutably
// FnMut: the closure captures by mutable reference (&mut T)
// FnOnce: the closure captures by value (T), takes ownership of the variables it captures from its enclosing scope

// Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce)

fn do_twice<T>(f: T, arg: i32) -> i32
where
    T: Fn(i32) -> i32,
{
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);


    // Using closure
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();

    println!("{:?}", list_of_strings);


    // tuple struct and enum
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20)
        .map(Status::Value)
        .collect();
}

// returning closures from functions
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}