#![allow(unused)]

// Modules organize code and define Rust's privacy boundaries

// mod <Name_of_Mod> ; Tells Rust to load module from a file named
mod front_of_house;
mod back_house;


// use crate::front_of_house::hosting;
use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    
    // Relative path
    front_of_house::hosting::add_to_waitlist();


    // Order a breakfast in the summer with Rye toast
    let mut meal = back_house::Breakfast::summer("rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");


    // We can access a public enum variants
    let order1 = back_house::Appetizer::Soup;
    let order2 = back_house::Appetizer::Salad;


    // Because use front_of_house::hosting is added to scope of this
    // then we can do this:hosting
    hosting::add_to_waitlist();
}

fn serve_order() {}


