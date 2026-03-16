use serde::Deserialize;
use serde_json::Value;

#[derive(Debug)]
pub struct Plat {
    pub restaurant: String,
    pub type_menu: String,
    pub name: String,
    pub category: String
}
