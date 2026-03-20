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
    let response = ApiClient::get().await.unwrap();
    let cafeterias: Vec<Cafeteria> = response.json().await.unwrap();
    let mut dishes: Vec<Dish> = filter_menu(cafeterias);
    if food_type.is_empty() {
        let message = format!("Please say what do you wnat to eat in the command (usage : !yum [filter])\n");
        return message;
    }
    return "restaurant".to_string();
}
