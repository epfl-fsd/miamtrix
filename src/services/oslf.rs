use crate::models::dish::Dish;
use crate::utils::cache::get_cached_dishes;
use crate::utils::message::message;

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
    let mut dishes: Vec<Dish> = match get_cached_dishes().await {
        Ok(d) => d,
        Err(_) => return format!("Sorry, Failed to load dish")
    };

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
