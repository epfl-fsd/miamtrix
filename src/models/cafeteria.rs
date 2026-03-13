use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cafeteria {
    pub name: String,
    pub menu_lines: Vec<Menu>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Menu {
    pub meals: Vec<Meal>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Meal {
    pub items: Vec<Item>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub recipe: Recipe
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    pub name: String
}
