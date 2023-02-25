use crate::front_of_house::serving;

fn fix_incorrect_order(){
    cook_order();
    // super::serve_order();
    // serving::serve_order();  // it is the same as above super
    super::back_of_house::serving::serve_order(); // it is the same as above super// 
}

fn cook_order(){}

pub struct Breakfast{
    pub toast: String,
    season_fruit: String,
}

impl Breakfast{
    pub fn sumer(toast: &str) -> Self{
        Self{
            toast: String::from(toast),
            season_fruit: String::from("peaches"),
        }
    }
    pub fn get_summer_fruit(&self) -> &str{
        &self.season_fruit
    }
}