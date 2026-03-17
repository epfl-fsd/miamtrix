use crate::utils::api::ApiClient;
use crate::models::{
    cafeteria::Cafeteria,
    dish::Dish,
};
use crate::utils::{
    filter_menu::filter_menu,
    message::message
};

pub async fn get_menu(command: &str) -> String {
    let response = ApiClient::get().await.unwrap();
    let (restaurant, filter) = get_restaurant_filter(command);
    let cafeteria: Vec<Cafeteria> = response.json().await.unwrap();
    let mut dishes: Vec<Dish> = filter_menu(cafeteria);
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
