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

pub async fn get_restaurant(food_type: &str) -> String {
    let search = food_type.trim().to_lowercase();
    if search.is_empty() {
        return "Please specify what you want to eat (or avoid)!\n✅ Include: !yum [food] (e.g., !yum pizza)\n❌ Exclude: !yum ![food] (e.g., !yum !fish)\n".to_string();
    };

    let response = match ApiClient::get().await {
      Ok(resp) => resp,
      Err(_) => return "Error, could not reach the restaurant API.".to_string(),
    };

    let cafeterias: Vec<Cafeteria> = match response.json().await {
        Ok(data) => data,
        Err(_) => return "Error, Failed to load menus data".to_string(),
    };

    let mut dishes: Vec<Dish> = filter_menu(cafeterias);

    let (is_exclusion, term) = if search.starts_with('!') {
        (true, &search[1..])
    } else {
        (false, search.as_str())
    };

    dishes.retain(|d| {
        let name = d.name.to_lowercase();
        let restaurant = d.restaurant.to_lowercase();
        let r#type = d.menu_type.to_lowercase();

        let contains_term = name.contains(term)
            ||r#type.contains(term)
            || restaurant.contains(term);

        if is_exclusion {
            !contains_term
        } else {
            contains_term
        }
    });
    message(dishes)
}
