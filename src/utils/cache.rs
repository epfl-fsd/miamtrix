use crate::models::{
    dish::Dish,
    cafeteria::Cafeteria
};
use crate::utils::{
    api::ApiClient,
    filter_menu::filter_menu,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Instant, Duration};

pub struct DishCache {
    pub last_update: Option<Instant>,
    pub dishes: Arc<Vec<Dish>>,
}
pub type SharedCache = Arc<RwLock<DishCache>>;

pub fn create_cache() -> SharedCache {
    Arc::new(RwLock::new(DishCache {
        last_update: None,
        dishes: Arc::new(Vec::new()),
    }))
}

const CACHE_TTL: Duration = Duration::from_secs(30 * 60);

pub async fn get_cached_dishes(cache: &SharedCache, api: &ApiClient) -> Result<Arc<Vec<Dish>>, String> {
    {
        let cache_read = cache.read().await;

        if let Some(last_time) = cache_read.last_update {
            if last_time.elapsed() < CACHE_TTL && !cache_read.dishes.is_empty() {
                return Ok(Arc::clone(&cache_read.dishes));
            }
        }
    }
    let mut cache_write = cache.write().await;

    if let Some(last_time) = cache_write.last_update {
        if last_time.elapsed() < CACHE_TTL && !cache_write.dishes.is_empty() {
            return Ok(Arc::clone(&cache_write.dishes));
        }
    }
    let response = api.get()
        .await
        .map_err(|e| format!("Failed to reach restaurant api: {}", e))?;

    let cafeterias: Vec<Cafeteria> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse data: {}", e))?;

    let dishes = Arc::new(filter_menu(cafeterias));

    cache_write.last_update = Some(Instant::now());
    cache_write.dishes = Arc::clone(&dishes);

    Ok(dishes)
}
