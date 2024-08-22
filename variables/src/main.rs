fn main() {
    let x:i32 = 5;
    println!("The value of x is: {}", x);
    let x = "six";// shadowing
    println!("The value of x is: {}", x);

    const SPEED_OF_LIGHT:f64 = 299_792_458.0;
    println!("The speed of light is: {} m/s", SPEED_OF_LIGHT);

    // Scalar Types(primitive types)
    // integer
    let a:i32 = 99_123; // decimal
    let b:i32  = 0xff;// hexadecimal
    let c:i32 = 0o77;// octal
    let d:i32 = 0b1111_0000;// binary
    let e:u8 = b'A';// byte (u8 only)

    let f:u8 = 255; // 256 will cause overflow

    // floating point 
    let g = 2.0; // f64 default
    let h:f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;// 1.759006211180124
    // remainder
    let remainder = 43 % 5; // 3

    // boolean
    let t = true;
    let f = false;

    // character
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';



    // Compound Types
    // tuple
    let tup:(i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; //destructuring
    let sub = tup.0; // dot notation, access by index

    // array
    let error_codes = [404, 500, 200];
    let not_found = error_codes[0];
    let server_error = error_codes[1];
    let success = error_codes[2];
    //let x = error_codes[3]; // panic, out of bound

    let a = [3; 5]; // [3, 3, 3, 3, 3]

   
    // function
    let sum = my_function(11, 22);
    println!("The sum is: {}", sum);

    // control flow
    // if
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else if number == 5 {
        println!("second condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    //loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter; // break the loop and return the value
        }
        
    };
    println!("The result is: {}", result);

    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    // iterate over array
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    // for loop with range
    for number in (1..4).rev() { //reverse 3, 2, 1
        println!("reverse for loop, {}", number);
    }

    // Comments

    // single line comment

    /*
    multi line comment
    */

    



} 
// function
fn my_function(x:i32, y:i32)->i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    let sum = x + y;
    sum
}
