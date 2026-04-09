use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Dish {
    pub restaurant: Arc<str>,
    pub location: Arc<str>,
    pub menu_type: Arc<str>,
    pub name: Arc<str>,
    pub category: Arc<str>,
    pub alergen: Vec<Arc<str>>
}
