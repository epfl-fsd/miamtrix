use crate::utils::api::ApiClient;
use crate::models::{
    cafeteria::Cafeteria,
    dish::Dish,
};
use crate::utils::{
    filter_menu::filter_menu,
    message::message
};

pub async fn get_fries(args: &str) -> String {
    let mut city = "".to_string();
    let mut iter = args.split_whitespace();

    while let Some(word) = iter.next() {
        match word {
            "-c" | "--city" => {
                if let Some(d) = iter.next() {
                    city = d.to_string();
                }
            }
            _ => {
                break;
            }
        }
    }
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
        let location = d.location.to_lowercase();
        let name = d.name.to_lowercase();
        let r#type = d.menu_type.to_lowercase();
        search.iter().any(|&term| {
            (name.contains(term) || r#type.contains(term)) && location.contains(&city)
        })
    });
    message(dishes)
}
