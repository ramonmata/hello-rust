fn fix_incorrect_order() {
    cook_order();
    super::serve_order();
}

fn cook_order() {}

// Public struct but its fields keep as private
// need to add pub to the desired fields we want public
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}


// Public enum makes all its variants public automatically
pub enum Appetizer {
    Soup,
    Salad
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast{
        Breakfast {
            toast: toast.to_string(),
            seasonal_fruit: String::from("peaches"),
        }
    }
}