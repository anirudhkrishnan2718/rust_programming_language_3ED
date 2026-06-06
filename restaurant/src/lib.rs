// module named crate is the implicit parent module of everything in lib.rs
// moved to front_of_house.rs in same directory
mod front_of_house;

// by convention, the module containing the function is brought into scope,
// not the function itself.
use crate::front_of_house::hosting;
fn deliver_order() {}

use std::fmt::Result;

// overwrites the above function
// use std::io::Result;

// use the as keyword to define distinct aliases for two functions from different
// modules that share a name
use std::io::Result as IoResult;

// curly brackets to group together imports that share a common tree structure
use std::{cmp::Ordering, io};
// self is a special keyword here that means the existing module
// use std::io::{self, Write};
// The asterisk is a wildcard that brings all public items into scope

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super refers to one level above the current module in the tree
        // in this case, it is the crate root level
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        // only the first field is public
        seasonal_fruit: String,
    }

    // since seasonal_fruit is private, a public associated function is needed
    // to create instances of Breakfast (cannot directly assign seasonal_fruit)
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // pub enums automatically mean all their variants are public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    //relative path
    front_of_house::hosting::add_to_waitlist();
    // use keyword has brought hosting into scope
    hosting::add_to_waitlist();
}

pub fn eat_at_restaurant_bf() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // meal.toast is mutable
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // cannot set or get seasonal_fruit, uncommenting next line will throw error
    // meal.seasonal_fruit = String::from("blueberries");
}

pub fn eat_at_restaurant_app() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
