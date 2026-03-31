use crate::utils::api::ApiClient;
use deunicode::deunicode;
use crate::models::{
    cafeteria::Cafeteria,
    dish::Dish,
};
use crate::utils::{
    filter_menu::filter_menu,
    message::message
};

pub async fn get_restaurant(args: &str) -> String {
    let mut search = String::new();
    let mut city = String::new();
    let mut allergen = String::new();
    let mut iter = args.split_whitespace();

    while let Some(word) = iter.next() {
        match word {
            "-c" | "--city" => {
                if let Some(d) = iter.next() {
                    city = d.to_string();
                }
            }
            "-s" | "--search" => {
                if let Some(d) = iter.next() {
                    search = d.to_string()
                }
            }
            "-a" | "--allergen" => {
                if let Some(d) = iter.next() {
                    allergen = d.to_string()
                }
            }
            _ => {
                continue;
            }
        }
    }
    if search.is_empty() {
        return "Please specify what you want to eat (or avoid)!\n✅ Include: !yum -s [food] (e.g., !yum -s pizza)\n❌ Exclude: !yum -s ![food] (e.g., !yum -s !fish)\n".to_string();
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

    let term_lower = term.to_lowercase();
    let allergen_lower = allergen.to_lowercase();
    let city_normalized = deunicode(&city).to_lowercase();
    dishes.retain(|d| {

        if !city.is_empty() {
            let location = deunicode(&d.location).to_lowercase();
            if location != city_normalized {
                return false;
            }
        }
        if !allergen.is_empty() {
            let contains_alergen = d.alergen.iter().any(|al| {
                let al_lower = al.to_lowercase();
                al_lower.contains(&allergen_lower) || al_lower == "alergen not specified"
            });
            if contains_alergen {
                return false;
            }
        }
        let name = d.name.to_lowercase();
        let restaurant = d.restaurant.to_lowercase();
        let r#type = d.menu_type.to_lowercase();

        let contains_term = name.contains(&term_lower)
            ||r#type.contains(&term_lower)
            || restaurant.contains(&term_lower);

        if is_exclusion {
            !contains_term
        } else {
            contains_term
        }
    });
    message(dishes)
}
