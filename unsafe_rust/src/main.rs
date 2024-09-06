use std::slice;

extern "C" {
    fn abs(input: i32) -> i32;
}

// Accessing or modifying a mutable static variable
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// ability to implement unsafe traits
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {
    // Dereferencing a raw pointer
    let mut num = 5;

    let r1 = &num as *const i32; // immutable raw pointer
    let r2 = &mut num as *mut i32; // mutable raw pointer

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Calling an unsafe function or method
    unsafe fn dangerous() {}
    
    unsafe {
        dangerous();
    }

    // creating a safe abstraction over unsafe code
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let(r1, r2) = r.split_at_mut(3);
    assert_eq!(r1, &mut [1, 2, 3]);
    assert_eq!(r2, &mut [4, 5, 6]);

    // Using extern functions to call external code
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Accessing or modifying a mutable static variable
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // ability to access fields of unions
    union IntOrFloat {
        i: i32,
        f: f32,
    }

    let mut iof = IntOrFloat { i: 123 };


}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}