use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cafeteria {
    pub name: String,
    pub menu_lines: Vec<Menu>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Menu {
    pub meals: Vec<Meal>,
    pub name: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Meal {
    #[serde(default)]
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
    pub name: String,
    pub category: String,
    #[serde(default)]
    pub labels: Vec<Label>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub label: String,
}
