fn fix_incorrect_order(){
    cook_order();
    super::serve_order();
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