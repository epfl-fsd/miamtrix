use crate::models::{
    dish::Dish,
    cafeteria::Cafeteria
};

pub fn filter_menu(cafeterias: Vec<Cafeteria>) -> Vec<Dish> {
    let mut plats = Vec::new();

    for resto in cafeterias {
        let cafet_name = resto.name.trim();

        for menu in resto.menu_lines {
            let menu_name = menu.name.trim();
            if menu.meals.is_empty() {
                plats.push(Dish {
                    restaurant: cafet_name.trim().to_string(),
                    menu_type: menu_name.to_string(),
                    name: menu_name.to_string(),
                    category: "unclassified".to_string(),
                    alergen: vec!["alergen not specified".to_string()]
                });
                continue;
            }

            for meal in menu.meals {
                for item in meal.items {
                    plats.push(Dish {
                        restaurant: cafet_name.trim().to_string(),
                        menu_type: menu.name.trim().to_string(),
                        name: item.recipe.name.trim().to_string(),
                        category: item.recipe.category.trim().to_string(),
                        alergen: {
                            let extracted_labels: Vec<String> = item.recipe.labels
                                .into_iter()
                                .map(|l| l.label.trim().to_string())
                                .collect();
                            if extracted_labels.is_empty() {
                                vec!["alergen not specified".to_string()]
                            } else {
                                extracted_labels
                            }
                        },
                    })
                }
            }
        }
    }
    plats
}
