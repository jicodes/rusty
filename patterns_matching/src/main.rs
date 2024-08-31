fn main() {
    // Patterns and Match Expression

    #[derive(Debug)]
    enum Language {
        English,
        Spanish,
        French,
        Japanese,
    }

    let language = Language::English;
    match language {
        Language::English => println!("Hello!"),
        Language::Spanish => println!("Hola!"),
        Language::French => println!("Bonjour!"),
        // _ => println!("I don't know this language"),
        lang => println!("I don't know this language: {:?}", lang),
    }

    // Conditional if let Expressions
    // if let PATTERN = EXPRESSION {
    //  code to execute if the pattern matches
    // } else {
}

    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    if let Some(status) = authorization_status {
        println!("Authorization status: {}", status);
    } else if is_admin {
        println!("Admin user");
    } else if let Ok(id) = group_id {
        if id > 30 {
            println!("Authorization status: privileged");
        } else {
            println!("Authorization status: basic");
        }
    } else {
        println!("Authorization status: guest");
    }

    // While let Conditional Loops
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // For Loops
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // Let Statements
    // Let PATTERN = EXPRESSION;
    let x = 5;
    let (x, y, z) = (1, 2, 3);

    println!("x: {}, y: {}, z: {}", x, y, z);

    // Function Parameters

    let point = (3, 5);
    print_coordinates(&point);



    // Irrefutable Patterns
    // patterns that cannot be simplified or broken down further. 
    let x = 5;

    // Refutable Patterns
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("{}", x);
    } else {
        println!("x is None");
    }

    // Can only accept irreducible patterns:
    // function parameters
    // let statements
    // for loops 
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
