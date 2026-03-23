use crate::config::CONFIG;
use std::sync::OnceLock;
use reqwest::Client;
use chrono::Local;

pub struct ApiClient {
    base_url: String,
    api_password: String,
    api_username: String,
    client: Client,


}
pub static API: OnceLock<ApiClient> = OnceLock::new();

impl ApiClient {
    pub fn init() {
        let config = CONFIG.get().expect("Please, load the config before the loading of the client api.");
        let instance = ApiClient {
            base_url: config.api_uri.clone(),
            api_password: config.api_password.clone(),
            api_username: config.api_username.clone(),
            client: Client::new(),
        };
        let _ = API.set(instance);
    }
    pub async fn get() -> Result<reqwest::Response, reqwest::Error> {
        let api = API.get().expect("Client api not initialised.");
        let date = Local::now().format("%Y-%m-%d").to_string();
        println!("{}",date);
        api.client
            .get(format!("{}?date={}", api.base_url, date))
            .basic_auth(&api.api_username, Some(&api.api_password))
            .send()
            .await
    }
}
