use crate::models::{
    dish::Dish,
    cafeteria::Cafeteria
};
use crate::utils::{
    api::ApiClient,
    filter_menu::filter_menu,
};
use std::sync::OnceLock;
use tokio::sync::RwLock;
use std::time::{Instant, Duration};

pub struct DishCache {
    pub last_update: Option<Instant>,
    pub dishes: Vec<Dish>,
}
pub static CACHE: OnceLock<RwLock<DishCache>> = OnceLock::new();

const CACHE_TTL: Duration = Duration::from_secs(30 * 60);

pub async fn get_cached_dishes() -> Result<Vec<Dish>, String> {
    let cache_lock = CACHE.get_or_init(|| {
        RwLock::new(DishCache {
            last_update: None,
            dishes: Vec::new(),
        })
    });

    {
        let cache_read = cache_lock.read().await;

        if let Some(last_time) = cache_read.last_update {
            if last_time.elapsed() < CACHE_TTL && !cache_read.dishes.is_empty() {
                return Ok(cache_read.dishes.clone());
            }
        }
    }

    let mut cache_write = cache_lock.write().await;

    if let Some(last_time) = cache_write.last_update {
        if last_time.elapsed() < CACHE_TTL && !cache_write.dishes.is_empty() {
            return Ok(cache_write.dishes.clone());
        }
    }
    let response = ApiClient::get()
        .await
        .map_err(|_| "Failed to reach restaurant api".to_string())?;

    let cafeterias: Vec<Cafeteria> = response
        .json()
        .await
        .map_err(|_| "Failed to parse data".to_string())?;

    let dishes = filter_menu(cafeterias);

    cache_write.last_update = Some(Instant::now());
    cache_write.dishes = dishes.clone();

    Ok(dishes)

}
