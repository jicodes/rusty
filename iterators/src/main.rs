// all iterators in Rust implement the Iterator trait
// pub trait Iterator {
//     type Item; // associated type, the type of the values the iterator will return
//     fn next(&mut self) -> Option<Self::Item>;

//     // methods with default implementations elide the need to implement them
// }

// The Iterator trait only requires implementors to define one method: the next method, which returns one item of the iterator at a time wrapped in Some and, when iteration is over, returns None.

// #[test]
// fn iterator_demonstration() {
//     let v1 = vec![1, 2, 3];
//     // let mut v1_iter = v1.iter(); // v1_iter is an iterator, returns immutable references
//     // let mut v1_iter = v1.ite_mut(); // returns mutable references
//     let mut v1_iter = v1.into_iter(); //  returns owned values

//     assert_eq!(v1_iter.next(), Some(1));
//     assert_eq!(v1_iter.next(), Some(2));
//     assert_eq!(v1_iter.next(), Some(3));
//     assert_eq!(v1_iter.next(), None);
// }

// #[test]
// fn iterator_sum() {
//     let v1 = vec![1, 2, 3];
//     let v1_iter = v1.iter();
//     let total: i32 = v1_iter.sum();
//     assert_eq!(total, 6);
// }
/*
    The sum method consumes the iterator v1_iter. This means that sum takes ownership of v1_iter and iterates over it internally to calculate the sum of its elements. Since sum consumes the iterator, you don't need to make v1_iter mutable.
 */


// derive Attribute: The derive attribute is a special attribute that automatically generates implementations of specified traits for a struct or enum. In this case, it is used to derive the PartialEq and Debug traits.
// PartialEq trait: allows you to use the == operator to compare values of this type. The trait requires that you also derive the Eq trait, which amounts to implementing an equality method that returns true when two values are equal and false when they are not.
// Debug trait: allows you to print the values of a struct in a way that is useful for developers so you can see the values while debugging your code.
// #[derive(PartialEq, Debug)]
// struct Shoe {
//     size: u32,
//     style: String,
// }

// fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
//     // let mut in_my_size: Vec<Shoe> = Vec::new();
//     // for shoe in shoes {
//     //     if shoe.size == shoe_size {
//     //         in_my_size.push(shoe);
//     //     }
//     // }
//     // in_my_size
//     shoes.into_iter()
//         .filter(|s| s.size == shoe_size)
//         .collect()
// }


// implementing the Iterator trait on a custom type
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// implementing the Iterator trait on the Counter struct
impl Iterator for Counter {
    type Item = u32;
    // only the next method needs to be implemented,
    // the rest of the methods have default implementations by the Iterator trait
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

// other methods provided by the Iterator trait
// using the Counter iterator with other Iterator trait methods
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        // zip creates an iterator that returns pairs of items from two iterators
        .zip(Counter::new().skip(1))  // skip skips the first n values and returns the rest
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}


fn main() {
    // let v1 = vec![1, 2, 3];
    // let v1_iter = v1.iter();
    // for val in v1_iter {
    //     println!("Got: {}", val);
    // }

    // iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up
    // let v1: Vec<i32> = vec![1, 2, 3];
    // let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    // assert_eq!(v2, vec![2, 3, 4]);


}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn filters_by_size() {
//         let shoes = vec![
//             Shoe { size: 10, style: String::from("sneaker") },
//             Shoe { size: 13, style: String::from("sandal") },
//             Shoe { size: 10, style: String::from("boot") },
//         ];

//         let in_my_size = shoes_in_my_size(shoes, 10);
//         assert_eq!(
//             in_my_size,
//             vec![
//                 Shoe { size: 10, style: String::from("sneaker") },
//                 Shoe { size: 10, style: String::from("boot") },
//             ]
//         );
//     }
// }