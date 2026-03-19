use crate::models::dish::Dish;
use std::fmt::Write;    

pub fn message(dishes: Vec<Dish>) -> String {
    let mut message = format!("# Daily menu :\n");
    for dish in dishes {
        let _ = writeln!(message, " - Dish: *{}*\n", dish.name);
        let _ = writeln!(message, "Restaurant: *{}*\n", dish.restaurant);
        let _ = writeln!(message, "Type: *{}*\n", dish.category);
    }
    message
}
