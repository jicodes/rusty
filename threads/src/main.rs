use std::{thread, time::Duration};

fn main() {
    // spawn a new thread
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // main thread ends, spawned thread stops
    // for i in 1..5 {
    //     println!("hi number {} from the main thread", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // wait for the spawned thread to finish
    // handle.join().unwrap();


    // move values into the spawned thread
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
    
}
