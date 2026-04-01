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
    let response = ApiClient::get().await.unwrap();
    let raw_bytes = match response.bytes().await {
        Ok(b) => b,
        Err(_) => return "Error, Failed to load menu data".to_string(),
    };
    let cafeterias: Vec<Cafeteria> = match serde_json::from_slice(&raw_bytes) {
        Ok(data) => data,
        Err(_) => return "Error, Failed to parse menus data".to_string(),
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
