use crate::models::dish::Dish;

pub fn message(dishes: Vec<Dish>) -> String {
    let mut message = format!("# Menu du jour\n");
    for dish in dishes {
        message.push_str(&format!(" - Nom: `{}`\n", dish.name));
        message.push_str(&format!("Restaurant: `{}`\n", dish.restaurant));
        message.push_str(&format!("Type: # {}\n", dish.category));
    }
    message
}
