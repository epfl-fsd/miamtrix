use crate::utils::api::ApiClient;
use std::fmt::Write;
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
    let raw_bytes = match response.bytes().await {
        Ok(b) => b,
        Err(_) => return "Error, Failed to load menu data".to_string(),
    };
    let cafeterias: Vec<Cafeteria> = match serde_json::from_slice(&raw_bytes) {
        Ok(data) => data,
        Err(_) => return "Error, Failed to parse menus data".to_string(),
    };
    if command.is_empty() {
        let mut message = format!("Please put a restaurant in the command (usage : !menu [restaurant])\n");
        let _ = writeln!(message, "### Restaurant list :\n");
        for resto in cafeterias {
            let _ = writeln!(message, "- {}\n", &resto.name);
        }
        return message;
    }
    let (restaurant, filter) = get_restaurant_filter(command);
    let mut dishes: Vec<Dish> = filter_menu(cafeterias);
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
