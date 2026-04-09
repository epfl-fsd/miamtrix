use crate::models::dish::Dish;
use crate::utils::message::message;
use crate::utils::cache::get_cached_dishes;
use crate::SharedCache;
use crate::ApiClient;
use std::sync::Arc;


pub async fn get_menu(command: &str, cache: &SharedCache, api: &ApiClient) -> String {
    if command.is_empty() {
         return "Please put a restaurant in the command (usage : !menu [restaurant])\n Run `!list` command to list all restaurant".to_string()
    }
    let dishes: Arc<Vec<Dish>> = match get_cached_dishes(cache, api).await {
        Ok(d) => d,
        Err(e) => {
            log::warn!("Failed to load dishes in menu command : {}", e);
            return format!("Sorry, failed to load dishes: {}", e);
        }
    };
    let (restaurant, filter) = get_restaurant_filter(command);
    let filtered_dishes: Vec<&Dish> = if !restaurant.is_empty() {
        dishes.iter()
            .filter(|d| d.restaurant.to_lowercase().contains(&restaurant) && d.name.to_lowercase().contains(&filter))
            .collect()
    } else {
        dishes.iter().collect()
    };
    let message: String = message(filtered_dishes);
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
