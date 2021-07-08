pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("Peaches"),
        }
    }
}

fn fix_incorrect_order(){
    cook_order();
}

pub enum Appetizer {
    Soup,
    Salad,
}

fn cook_order(){}