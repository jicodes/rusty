use std::sync::mpsc; // multiple producer, single consumer
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel(); // tx is the sending end, rx is the receiving end
    let tx2 = mpsc::Sender::clone(&tx); // clone the sending end

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap(); // send val to the receiving end
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx2.send(val).unwrap(); // send val to the receiving end
            thread::sleep(Duration::from_secs(1));
        }
    });

    // let received = rx.try_recv().unwrap(); // non-blocking, returns a Result<T, E>
    for received in rx {
        println!("Got: {}", received);
    }

}
