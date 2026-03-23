use crate::utils::api::ApiClient;
use crate::models::{
    cafeteria::Cafeteria,
    dish::Dish,
};
use crate::utils::{
    filter_menu::filter_menu,
    message::message
};

pub async fn get_fries() -> String {
    let response = match ApiClient::get().await {
      Ok(resp) => resp,
      Err(_) => return "Error, could not reach the restaurant API.".to_string(),
    };

    let cafeterias: Vec<Cafeteria> = match response.json().await {
        Ok(data) => data,
        Err(_) => return "Error, Failed to load menus data".to_string(),
    };

    let mut dishes: Vec<Dish> = filter_menu(cafeterias);

    let search = ["fries", "frite"];
    dishes.retain(|d| {
        let name = d.name.to_lowercase();
        let r#type = d.menu_type.to_lowercase();
        search.iter().any(|&term| {
            name.contains(term) || r#type.contains(term)
        })
    });
    message(dishes)
}
