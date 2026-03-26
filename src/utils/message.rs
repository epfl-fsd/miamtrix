use crate::models::dish::Dish;
use std::fmt::Write;
use std::collections::BTreeMap;

pub fn message(dishes: Vec<Dish>) -> String {
    let mut message = String::from("# Daily menu :\n\n");
    let mut grouped_data: BTreeMap<String, BTreeMap<String, Vec<Dish>>> = BTreeMap::new();

    for dish in dishes {
        let location = dish.location.clone();
        let restaurant = dish.restaurant.clone();
        grouped_data
            .entry(location)
            .or_default()
            .entry(restaurant)
            .or_default()
            .push(dish);
    }
    for (location, restaurant) in grouped_data {
        let _ = writeln!(message, "## {}", location);
        for (restaurant, dishes) in restaurant {
            let _ = writeln!(message, "### {}", restaurant);
            for dish in dishes {
                let _ = writeln!(message, "- **{}**", dish.name);
                let _ = writeln!(message, "  *Type: {}*", dish.menu_type);
            }
            let _ = writeln!(message);
        }
        let _ = writeln!(message, "---\n");
    }
    message
}
