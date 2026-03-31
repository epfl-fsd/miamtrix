use crate::models::dish::Dish;
use crate::utils::api::ApiClient;
use crate::models::cafeteria::Cafeteria;
use crate::utils::filter_menu::filter_menu;
use std::fmt::Write;
use std::collections::BTreeMap;
use deunicode::deunicode;

pub async fn list_restaurant(args: &str) -> String {
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
    let cafeterias: Vec<Cafeteria> = response.json().await.unwrap();
    let dishes: Vec<Dish> = filter_menu(cafeterias);
    let mut message = String::new();
    let mut grouped_data: BTreeMap<String, BTreeMap<String, Vec<Dish>>> = BTreeMap::new();

    for dish in dishes {
        let location = dish.location.clone();
        let restaurant = dish.restaurant.clone();
        grouped_data
            .entry(location)
            .or_default()
            .entry(restaurant)
            .or_default()
            .push(dish);
    }
    let _ = writeln!(message, "### Restaurant list :\n");
    let search_city = deunicode(&city).to_lowercase();
    for (location, restaurant) in grouped_data {
        let location_normalized = deunicode(&location).to_lowercase();
        if !search_city.is_empty() && location_normalized != search_city {
            continue;
        }
        let _ = writeln!(message, "### {}", location);
        for (restaurant, _dishes) in restaurant {
            let _ = writeln!(message, " - {}", restaurant);
        }
        let _ = writeln!(message, "---\n");
    }
    message
}
