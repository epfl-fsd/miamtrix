use crate::models::dish::Dish;
use crate::utils::cache::get_cached_dishes;
use crate::utils::message::message;
use crate::SharedCache;
use crate::ApiClient;
use std::sync::Arc;

pub async fn get_fries(args: &str, cache: &SharedCache, api: &ApiClient) -> String {
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
    let dishes: Arc<Vec<Dish>> = match get_cached_dishes(cache, api).await {
        Ok(d) => d,
        Err(e) => {
            log::warn!("Failed to load dishes in oslf command : {}", e);
            return format!("Sorry, failed to load dishes: {}", e);
        }
    };

    let search = ["fries", "frite"];
    let filtered_dishes: Vec<&Dish> = dishes.iter().filter(|d| {
        let location = d.location.to_lowercase();
        let name = d.name.to_lowercase();
        let r#type = d.menu_type.to_lowercase();
        search.iter().any(|&term| {
            (name.contains(term) || r#type.contains(term)) && location.contains(&city)
        })
    }).collect();
    message(filtered_dishes)
}
