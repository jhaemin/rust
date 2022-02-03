#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

use crate::front_of_house::hosting;
// use self::front_of_house::hosting; // use relative
// pub use crate::front_of_house::hosting; // re-exporting
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    let mut meal = back_of_house::Breakfast::summer("rye bread");
    meal.toast = String::from("wheat bread");
    println!("Give me the toast of {}", meal.toast);

    // `use crate::front_of_house::hosting;`
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // `use crate::front_of_house::hosting::add_to_waitlist;`
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peach"),
            }
        }
    }
}

use std::collections::HashMap;

fn hashMapTest() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

use std::fmt;
// use std::io;

use std::fmt::Result;
use std::io::Result as IoResult;

use rand::Rng;
fn randTest() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
}

// use std::io;
// use std::cmp::Ordering;
use std::{cmp::Ordering, io};
