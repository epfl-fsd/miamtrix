use crate::models::dish::Dish;
use crate::utils::cache::get_cached_dishes;
use std::fmt::Write;
use std::collections::BTreeMap;
use deunicode::deunicode;
use crate::SharedCache;
use crate::ApiClient;
use std::sync::Arc;


pub async fn list_restaurant(args: &str, cache: &SharedCache, api: &ApiClient) -> String {
    let mut city: Option<&str> = None;
    let mut iter = args.split_whitespace();

    while let Some(word) = iter.next() {
        match word {
            "-c" | "--city" => city = iter.next(),
            _ => {
                continue;
            }
        }
    }
    let dishes: Arc<Vec<Dish>> = match get_cached_dishes(cache, api).await {
        Ok(d) => d,
        Err(e) => {
            log::warn!("Failed to load dishes in list command : {}", e);
            return format!("Sorry, failed to load dishes: {}", e);
        }
    };
    let mut grouped_data: BTreeMap<std::sync::Arc<str>, BTreeMap<std::sync::Arc<str>, Vec<&Dish>>> = BTreeMap::new();

    for dish in dishes.iter() {
        let location = dish.location.clone();
        let restaurant = dish.restaurant.clone();
        grouped_data
            .entry(location)
            .or_default()
            .entry(restaurant)
            .or_default()
            .push(dish);
    }
    let search_city = city.map(|c| deunicode(c).to_lowercase());
    let mut message = String::with_capacity(500);
    let _ = writeln!(message, "### Restaurant list :\n");
    for (location, restaurants) in grouped_data {
        if let Some(ref target_city) = search_city {
            if deunicode(&location).to_lowercase() != *target_city {
                continue;
            }
        }
        let _ = writeln!(message, "### {}", location);
        for restaurant in restaurants {
            let _ = writeln!(message, " - {}", restaurant.0);
        }
        let _ = writeln!(message, "---\n");
    }
    if message.len() < 30 {
        return "No restaurants found for this location.".to_string();
    }
    message
}
