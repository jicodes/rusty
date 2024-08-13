// use std::fs::{self, File};
// use std::io::ErrorKind;
// use std::io;
// use std::io::Read;

use std::net::IpAddr;


fn main() {
    // panic!("");

    // Option enum
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // Result enum
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    // Result<T, E> is often used with the std::io module to handle I/O errors
    //let f = File::open("hello.txt");

    // shadowing f 
    // let f:File = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         match error.kind() {
    //             ErrorKind::NotFound => match File::create("hello.txt") {
    //                 Ok(fc) => fc,
    //                 Err(e) => panic!("Problem creating the file: {:?}", e),
    //             },
    //             other_error => {
    //                 panic!("Problem opening the file: {:?}", other_error)
    //             }
    //         }
    //     }
    // };

    // unwrap and expect
    // let f = File::open("hello.txt").unwrap();
    // let _f = File::open("hello.txt").expect("Failed to open hello.txt");


    // Error Propagation
    // fn read_username_from_file() -> Result<String, io::Error> {
        
        // let mut s = String::new();
        // // chaining method calls after the ? operator
        // File::open("hello.txt")?.read_to_string(&mut s)?;
        // Ok(s)

        // question mark operator can only be used in functions that have a return type of Result or Option


        // shorter version
        // fs::read_to_string("hello.txt")
    // }

    // unwrap and expect
    let home:IpAddr = "127.0.0.1".parse().unwrap();
    println!("Home IP: {}", home);

    // custom type for validation

    pub struct Guess {
        value: i32,
    }
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}", value);
            }
            Guess { value }
        }
        pub fn value(&self) -> i32 {
            self.value
        }
    }


}
