enum IpAddrKind {
    // variants with data
    V4(u8,u8,u8,u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Implement methods on an enum
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

// option enum
// enum Option<T> {
//     Some(T),
//     None,
// }

// match control flow operator
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,   
}


fn main() {
    let _four:IpAddrKind = IpAddrKind::V4(127,0,0,1);
    let _six:IpAddrKind = IpAddrKind::V6(String::from("::1"));

    let _localhost = IpAddrKind::V4(127,0,0,1);


    let _some_number:Option<i32> = Some(5);
    let _some_string:Option<&str> = Some("a string");
    let _absent_number: Option<i32> = None;

    // use Some and None directly without Option<T> prefix
    let x: i8 = 5;
    let y: Option<i8> = Some(5); // None also works
    let sum = x + y.unwrap_or(0);
    println!("sum: {}", sum);

    // match control flow operator
    value_in_cents(Coin::Quarter(UsState::Alaska));

    // match control flow operator with Option<T>
    let six = plus_one(Some(5));
    println!("six:{:?} ", six);

    // if let syntax of match
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("not three");
    }


}

// match control flow operator
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

// match control flow operator with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None, // match is exhaustive, so we can handle all cases using _
    }
}
