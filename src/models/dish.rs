use serde::Deserialize;
use serde_json::Value;

#[derive(Debug)]
pub struct Dish {
    pub restaurant: String,
    pub menu_type: String,
    pub name: String,
    pub category: String
}
