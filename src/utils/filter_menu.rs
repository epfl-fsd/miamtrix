use crate::models::{
    dish::Dish,
    cafeteria::Cafeteria
};

pub fn filter_menu(cafeterias: Vec<Cafeteria>) -> Vec<Dish> {
    let mut plats = Vec::new();

    for resto in cafeterias {
        let cafet_name = resto.name.trim();
        let cafet_location = match cafet_name {
            "Hopper" => "Lausanne",
            "Piano " => "Lausanne",
            "PUUR" => "Lausanne",
            "Montreux Jazz Café" => "Lausanne",
            "Epicure" => "Lausanne",
            "Esplanade" => "Lausanne",
            "Alpine" => "Lausanne",
            "Ginko" => "Lausanne",
            "Native-Restauration végétale - Bar à café" => "Lausanne",
            "Arcadie" => "Lausanne",
            "Cyber café SV by Novae" => "Lausanne",
            "Giacometti" => "Lausanne",
            "Niki " => "Lausanne",
            "Zaha" => "Lausanne",
            "Satellite" => "Lausanne",
            "Holy Cow - SwissTech Village" => "Lausanne",
            "Gina Ristorante" => "Lausanne",
            "Li Beirut - Food Truck" => "Lausanne",
            "Fleur de Pains - Food Truck" => "Lausanne",
            "Manira Wokshop - Food Truck Village" => "Lausanne",
            "Osteria 31 - Starling Hôtel EPFL" => "Lausanne",
            "NAS sandwiches" => "Lausanne",
            "Régal Tandoori - Food Truck" => "Lausanne",
            "Klee Compass" => "Lausanne",
            "Newrest Biotech Attitudes" => "Genève",
            "Newrest Biotech Crushly" => "Genève",
            "Industrie 21, Sion" => "Sion",
            "101Restos Microcity" => "Neuchâtel",
            "NAS Burger" => "Lausanne",
            "Pazza Pizza & Pasta" => "Lausanne",
            _ => "Lausanne"
        };
        for menu in resto.menu_lines {
            let menu_name = menu.name.trim();
            if menu.meals.is_empty() {
                plats.push(Dish {
                    restaurant: cafet_name.trim().to_string(),
                    menu_type: menu_name.to_string(),
                    location: cafet_location.to_string(),
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
                        location: cafet_location.to_string(),
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
