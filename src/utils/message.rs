use crate::models::dish::Dish;
use std::fmt::Write;
use std::collections::BTreeMap;

pub fn message(dishes: Vec<Dish>) -> String {
    let mut message = String::from("# Daily menu :\n\n");
    let mut group_restaurant: BTreeMap<String, Vec<Dish>> = BTreeMap::new();

    for dish in dishes {
        group_restaurant.entry(dish.restaurant.clone()).or_default().push(dish);
    }
    for (restaurant, dishes) in group_restaurant {
        let _ = writeln!(message, "### {}", restaurant);
        for dish in dishes {
            let _ = writeln!(message, "- **{}**", dish.name);
            let _ = writeln!(message, "  *Type: {}*", dish.menu_type);
        }
        let _ = writeln!(message);
    }
    message
}
