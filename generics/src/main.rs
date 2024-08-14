// struct Point<T> {
//     x: T,
//     y: T,
// }

// An implementation block (or impl block) in Rust is used to define methods and associated functions for a struct, enum, or trait. 
// It allows you to add behavior to your types by implementing methods that can operate on instances of those types.
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }


// impl Point<f64> {
//     fn y(&self) -> &f64 {
//         &self.y
//     }
// }


struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}





fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = get_largest(number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = get_largest(char_list);
    println!("The largest char is {}", result);


    let p1 = Point { x: 5, y: 10.5 };
    let p2 = Point { x: "hello", y: "world" };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // enum using generics
    enum Option<T> {
        Some(T),
        None,
    }

    let integer = Option::Some(5);
    let float = Option::Some(5.0);

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

}

fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}