// use std::thread;
// use std::time::Duration;

// // fn simulated_expensive_calculation(intensity: u32) -> u32 {
// //     println!("calculating slowly...");
// //     thread::sleep(Duration::from_secs(2));
// //     intensity
// // }

// fn main() {
//     let simulated_user_specified_value = 10;
//     let simulated_random_number = 7;

//     generate_workout(simulated_user_specified_value, simulated_random_number);
// }

// struct Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     calculation: T,
//     value: Option<u32>,
// }

// impl <T> Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     fn new(calculation: T) -> Cacher<T> {
//         Cacher {
//             calculation,
//             value: None,
//         }
//     }

//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.calculation)(arg);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }



// fn generate_workout(intensity: u32, random_number: u32) {
//     // closure definition could only have one concrete type inferred
//     let mut cacher_result = Cacher::new(|num| {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     });

//     if intensity < 25 {
//         println!(
//             "Today, do {} pushups!",
//             cacher_result.value(intensity)
//         );
//         println!(
//             "Next, do {} situps!",
//             cacher_result.value(intensity)
//         );
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 cacher_result.value(intensity)
//             );
//         }
//     }
// }

fn main() {
    // closure that captures its environment
    // let x = 4;
    // let equal_to_x = |z| z == x;
    // let y = 4;
    // assert!(equal_to_x(y)); // won't panic


    // force closure to take ownership of the values it uses in the environment by adding the move keyword
    // let x = vec![1, 2, 3];
    // let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x); // won't compile

    // let y = vec![1, 2, 3];
    // assert!(equal_to_x(y));


}

// FnOnce: takes ownership of the variables it captures from its enclosing scope
// could only be called once

// FnMut: borrows mutably
// Fn: borrows immutably