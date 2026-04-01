use crate::models::{
    dish::Dish,
    cafeteria::Cafeteria
};

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

        let cafet_name = cafet_name_str.to_string();
        let cafet_location = cafet_location_str.to_string();
        for menu in resto.menu_lines {
            let menu_name = menu.name.trim().to_string();
            if menu.meals.is_empty() {
                plats.push(Dish {
                    restaurant: cafet_name.clone(),
                    menu_type: menu_name.clone(),
                    location: cafet_location.clone(),
                    name: menu_name,
                    category: "unclassified".to_string(),
                    alergen: vec!["alergen not specified".to_string()]
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
                        restaurant: cafet_name.clone(),
                        menu_type: menu.name.clone(),
                        location: cafet_location.clone(),
                        name: item.recipe.name.trim().to_string(),
                        category: item.recipe.category.trim().to_string(),
                        alergen: if extracted_labels.is_empty() {
                                vec!["alergen not specified".to_string()]
                            } else {
                                extracted_labels
                            }
                    });
                }
            }
        }
    }
    plats
}
