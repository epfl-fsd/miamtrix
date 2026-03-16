use crate::models::{
    plat::Plat,
    cafeteria::Cafeteria
};

pub fn filter_menu(cafeterias: Vec<Cafeteria>) -> Vec<Plat> {
    let mut plats = Vec::new();

    for resto in cafeterias {
        let cafet_name = &resto.name;

        for menu in resto.menu_lines {
            if menu.meals.is_empty() {
                plats.push(Plat {
                    restaurant: cafet_name.clone(),
                    type_menu: menu.name.clone(),
                    name: menu.name.clone(),
                    category: "unclassified".to_string()
                });
                continue;
            }

            for meal in menu.meals {
                for item in meal.items {
                    plats.push(Plat {
                        restaurant: cafet_name.clone(),
                        type_menu: menu.name.clone(),
                        name: item.recipe.name.clone(),
                        category: item.recipe.category.clone(),
                    })
                }
            }
        }
    }
    plats
}
