use rand::{Rng, CryptoRng, ErrorKind::Transient};
use std::io::{self, Write};

mod front_of_house;

use self::front_of_house::hosting::add_to_waitlist;

fn eat_at_restaurant(){
    front_of_house::hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = "bread".to_string();
    let app = back_of_house::Appetizer::Salad;
    add_to_waitlist();

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("{}", secret_number);
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}

    pub enum Appetizer {
        Soup,
        Salad
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

fn serve_order(){

}

