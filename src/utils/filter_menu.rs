use crate::models::{
    dish::Dish,
    cafeteria::Cafeteria
};

pub fn filter_menu(cafeterias: Vec<Cafeteria>) -> Vec<Dish> {
    let mut plats = Vec::new();

    for resto in cafeterias {
        let cafet_name = &resto.name;

        for menu in resto.menu_lines {
            if menu.meals.is_empty() {
                plats.push(Dish {
                    restaurant: cafet_name.clone(),
                    menu_type: menu.name.clone(),
                    name: menu.name.clone(),
                    category: "unclassified".to_string()
                });
                continue;
            }

            for meal in menu.meals {
                for item in meal.items {
                    plats.push(Dish {
                        restaurant: cafet_name.clone(),
                        menu_type: menu.name.clone(),
                        name: item.recipe.name.clone(),
                        category: item.recipe.category.clone(),
                    })
                }
            }
        }
    }
    plats
}
