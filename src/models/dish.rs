#[derive(Debug)]
pub struct Dish {
    pub restaurant: String,
    pub menu_type: String,
    pub name: String,
    pub category: String,
    pub alergen: Vec<String>
}
