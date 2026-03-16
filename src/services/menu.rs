use crate::utils::api::ApiClient;
use crate::models::{
    cafeteria::Cafeteria,
    plat::Plat,
};
use crate::utils::{
    filter_menu::filter_menu,
    message::message
};

pub async fn get_menu(command: &str) -> String {
    let response = ApiClient::get().await.unwrap();
    let (restaurant, filter) = get_restaurant_filter(command);
    let cafeteria: Vec<Cafeteria> = response.json().await.unwrap();
    let mut plats: Vec<Plat> = filter_menu(cafeteria);
    if !restaurant.is_empty() {
        plats = plats.into_iter()
            .filter(|p| p.restaurant.to_lowercase().contains(&restaurant) && p.name.to_lowercase().contains(&filter))
            .collect();
    }
    let message: String = message(plats);
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
