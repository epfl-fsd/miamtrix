use crate::config::CONFIG;
use std::sync::OnceLock;
use reqwest::Client;

pub struct ApiClient {
    base_url: String,
    api_password: String,
    api_username: String,
    client: Client,


}
pub static API: OnceLock<ApiClient> = OncLock::new();

impl ApiClient {
    pub fn init() {
        let config = CONFIG.get().expect("la config doit etre chargé avant le client api");
        let instance = ApiClient {
            base_url: &config.api_uri.clone(),
            api_password: &config.api_password.clone(),
            api_username: &config.api_username.clone(),
            client: Client::new(),
        };
        API.set(instance).expect("le client api a déjà été initialisé")
    }
    pub async fn get() -> Result<(), reqwest::Error> {
        let api = API.get().expect("Le client api n'a pas été initialisé");
        let response = api.client
            .get(&api.base_url)
            .basic_auth(&api.api_username, Some(&api.api_password))
            .send()
            .await?;

        Ok(())
    }
}