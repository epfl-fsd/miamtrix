use crate::models::{
    dish::Dish,
    cafeteria::Cafeteria
};
use std::sync::Arc;

pub fn filter_menu(cafeterias: Vec<Cafeteria>) -> Vec<Dish> {
    let mut plats = Vec::with_capacity(cafeterias.len() * 15);

    for resto in cafeterias {
        let cafet_name_str = resto.name.trim();
        let cafet_location_str = match cafet_name_str {
            "Newrest Biotech Attitudes" | "Newrest Biotech Crushly" => "Genève",
            "Industrie 21, Sion" => "Sion",
            "101Restos Microcity" => "Neuchâtel",
            _ => "Lausanne"
        };

        let cafet_name: Arc<str> = Arc::from(cafet_name_str);
        let cafet_location: Arc<str> = Arc::from(cafet_location_str);

        for menu in resto.menu_lines {
            let menu_name: Arc<str> = Arc::from(menu.name.trim());
            if menu.meals.is_empty() {
                plats.push(Dish {
                    restaurant: Arc::clone(&cafet_name),
                    menu_type: Arc::clone(&menu_name),
                    location: Arc::clone(&cafet_location),
                    name: Arc::clone(&menu_name),
                    category: Arc::from("unclassified"),
                    alergen: vec![Arc::from("alergen not specified")],
                });
                continue;
            }

            for meal in menu.meals {
                for item in meal.items {
                    let extracted_labels: Vec<String> = item.recipe.labels
                        .into_iter()
                        .map(|l| l.label.trim().to_string())
                        .collect();

                    plats.push(Dish {
                        restaurant: Arc::clone(&cafet_name),
                        menu_type: Arc::clone(&menu_name),
                        location: Arc::clone(&cafet_location),
                        name: Arc::from(item.recipe.name.trim()),
                        category: Arc::from(item.recipe.category.trim()),
                        alergen: if extracted_labels.is_empty() {
                                vec![Arc::from("alergen not specified")]
                            } else {
                                extracted_labels.into_iter().map(|s| Arc::from(s.as_str())).collect()
                            }
                    });
                }
            }
        }
    }
    plats
}
