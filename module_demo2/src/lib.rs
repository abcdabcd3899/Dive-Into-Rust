
use rand::{Rng};
pub mod front_of_house;

pub use crate::front_of_house::hosting;  // pub  expose the external codes
use crate::front_of_house::serving::back_of_house::Breakfast;

pub fn eat_at_restaurant(){
    // absolute path. This is crate root is lib.rs, which is default crate root name
    hosting::add_to_waitlist();

    // relative path
    hosting::add_to_waitlist();

    let mut meal = Breakfast::sumer("Rye");
    meal.toast = String::from("Wheat");
    println!("{}", meal.get_summer_fruit());

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{}", secret_number);
}