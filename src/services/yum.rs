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
    if food_type.is_empty() {
        let message = format!("Please say what do you wnat to eat in the command (usage : !yum [filter])\n");
        return message;
    }
    let response = ApiClient::get().await.unwrap();
    let cafeterias: Vec<Cafeteria> = response.json().await.unwrap();
    let mut dishes: Vec<Dish> = filter_menu(cafeterias);

    dishes = dishes.into_iter()
        .filter(|d| {
            let name = d.name.to_lowercase();
            let restaurant = d.restaurant.to_lowercase();
            let r#type = d.menu_type.to_lowercase();
            return name.contains(&search) || r#type.contains(&search) || restaurant.contains(&search)
        })
        .collect();
    message(dishes)
}
