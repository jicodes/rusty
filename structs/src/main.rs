struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
// tuple struct
struct Color(i32, i32, i32);

// implement Debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// methods: implement methods for a struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// associated functions are functions that don’t take self as a parameter. They’re called associated functions because they’re associated with the struct. They’re still functions, not methods, because they don’t have an instance of the struct to work with.
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}


fn main() {
    // let mut user1 = User {
    //     email: String::from("user1@email.com"),
    //     username: String::from("user1"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // let name = user1.username;
    // user1.username = String::from("user11");

    // let user2 = build_user(
    //     String::from("user2@email.com"),
    //     String::from("user2"),
    // );

    // let user3 = User {
    //     email: String::from("user3@email.com"),
    //     username: String::from("user3"),
    //     ..user2
    // };
    // In Rust: Explicit fields in a struct are not overwritten by fields from the struct you’re spreading, regardless of their order.
	// In JavaScript: The spread operator will overwrite earlier properties with later ones. To avoid unwanted overwrites, place the spread operator before the explicit properties you want to retain.


    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let rectangle2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rectangle3 = Rectangle::square(20);

    // implement Display trait to print the struct
    println!("Rectangle is {:?}", rectangle);
    // beautiful print
    println!("Rectangle is {:#?}", rectangle);

    
    println!(
        "The area of the rectangle is {} square pixels.", 
        rectangle.area()
    );

    println!("Can rectangle hold rectangle2? {}", rectangle.can_hold(&rectangle2));


}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
