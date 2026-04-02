use crate::models::dish::Dish;
use crate::utils::message::message;
use crate::utils::cache::get_cached_dishes;

pub async fn get_menu(command: &str) -> String {
    if command.is_empty() {
         return "Please put a restaurant in the command (usage : !menu [restaurant])\n Run `!list` command to list all restaurant".to_string()
    }
    let mut dishes: Vec<Dish> = match get_cached_dishes().await {
        Ok(d) => d,
        Err(_) => return format!("Sorry, Failed to load dish")
    };
    let (restaurant, filter) = get_restaurant_filter(command);
    if !restaurant.is_empty() {
        dishes = dishes.into_iter()
            .filter(|d| d.restaurant.to_lowercase().contains(&restaurant) && d.name.to_lowercase().contains(&filter))
            .collect();
    }
    let message: String = message(dishes);
    message
}



fn get_restaurant_filter(params: &str) -> (String, String) {
    if params.is_empty() {
        return ("".to_string(), "".to_string());
    }
    let mut mots = params.split_whitespace();
    let restaurant = mots.next().unwrap_or("").to_lowercase();
    let filter = mots.next().unwrap_or("").to_lowercase();

    (restaurant, filter)
}
