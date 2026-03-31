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
    let mut search: Option<&str> = None;
    let mut city: Option<&str> = None;
    let mut allergen: Option<&str> = None;

    let mut iter = args.split_whitespace();

    while let Some(word) = iter.next() {
        match word {
            "-c" | "--city" => city = iter.next(),
            "-s" | "--search" => search = iter.next(),
            "-a" | "--allergen" => allergen = iter.next(),
            _ => {
                continue;
            }
        }
    }
    let search_term = match search {
        Some(s) if !s.is_empty() => s,
        _ => return "Please specify what you want to eat (or avoid)!\n✅ Include: !yum -s [food] (e.g., !yum -s pizza)\n❌ Exclude: !yum -s ![food] (e.g., !yum -s !fish)\n".to_string(),
    };

    let (is_exclusion, term) = if let Some(stripped) = search_term.strip_prefix('!') {
        (true, stripped)
    } else {
        (false, search_term)
    };

    let term_lower = term.to_lowercase();
    let target_allergen = allergen.map(|a| a.to_lowercase());
    let target_city = city.map(|c| deunicode(c).to_lowercase());

    let response = match ApiClient::get().await {
      Ok(resp) => resp,
      Err(_) => return "Error, could not reach the restaurant API.".to_string(),
    };

    let cafeterias: Vec<Cafeteria> = match response.json().await {
        Ok(data) => data,
        Err(_) => return "Error, Failed to load menus data".to_string(),
    };

    let mut dishes: Vec<Dish> = filter_menu(cafeterias);


    dishes.retain(|d| {
        if let Some(ref c) = target_city {
            if deunicode(&d.location).to_lowercase() != *c {
                return false;
            }
        }
        if let Some(ref a) = target_allergen {
            let contains_alergen = d.alergen.iter().any(|al| {
                if al == "alergen not specified" {
                    return true;
                }
                al.to_lowercase().contains(a)
            });
            if contains_alergen {
                return false;
            }
        }
        let contains_term = d.name.to_lowercase().contains(&term_lower)
            ||d.menu_type.to_lowercase().contains(&term_lower)
            || d.restaurant.to_lowercase().contains(&term_lower);

        if is_exclusion {
            !contains_term
        } else {
            contains_term
        }
    });
    message(dishes)
}
