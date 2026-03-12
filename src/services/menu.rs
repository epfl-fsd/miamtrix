use crate::utils::api::ApiClient;
use crate::models::cafeteria::{Cafeteria, Item};

pub async fn get_menu(command: &str) -> String {
    let response = ApiClient::get().await.unwrap();
    let (restaurant, filter) = get_restaurant_filter(command);
    let cafeteria: Vec<Cafeteria> = response.json().await.unwrap();

    let menus = cafeteria.into_iter().find(|c| {
        c.name.to_lowercase().contains(&restaurant.to_lowercase())
    });

    match menus {
        Some(resto) => {
            let mut message = format!("# Menu du jour pour le restaurant {} \n", resto.name);
            for menu in resto.menu_lines {
                for meal in menu.meals {
                    let list_plat: Vec<Item> = meal.items
                        .into_iter()
                        .filter(|n| n.recipe.name.to_lowercase().contains(&filter))
                        .collect();
                    for item in list_plat {
                        message.push_str(&format!(" - **{}**\n",item.recipe.name))
                    }
                }
                message.push_str("\n");
            }
            message
        },
        None => {
            format!("aucun resto trouvé pour {}", restaurant)
        }
    }
}



fn get_restaurant_filter(params: &str) -> (String, String) {
    let mut mots = params.split_whitespace();
    let restaurant = mots.next().unwrap_or("").to_lowercase();
    let filter = mots.next().unwrap_or("").to_lowercase();

    (restaurant, filter)
}
