use rand::{Rng, Error, CryptoRng};

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

mod front_of_house;

// use self::front_end_house::hosting;
pub use crate::front_of_house::hosting;

use std::fmt::Result;
use std::io::Result as IoResult;

use std::io::{self, Write};

use std::io::*;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}



// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_end_house::hosting::add_to_waitlist();

//     // Relative path
//     front_end_house::hosting::add_to_waitlist();
// }

fn serve_order() {}

pub mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }


    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye"); 
    meal.toast = String::from("Wheat");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    
    //front_end_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();


    // create a random number
    let secret_number = rand::thread_rng().gen_range(1..101);

}