use std::env;
use std::sync::OnceLock;

#[derive(Debug)]
pub struct AppConfig {
    pub api_uri: String,
    pub api_username: String,
    pub api_password: String,
    pub bot_username: String,
    pub bot_password: String,
    pub url_server_matrix: String,
}

pub static CONFIG: OnceLock<AppConfig> = OnceLock::new();

impl AppConfig {
    pub fn load_env() -> Self {
        AppConfig {
            api_uri: env::var("URI_API").expect("Variable manquante"),
            api_username: env::var("API_USERNAME").expect("Variable manquante"),
            api_password: env::var("API_PASSWORD").expect("Variable manquante"),
            bot_username: env::var("USERNAME_BOT").expect("Variable manquante"),
            bot_password: env::var("PASSWORD_BOT").expect("Variable manquante"),
            url_server_matrix: env::var("URL_SERVER_MATRIX").expect("Variable manquante"),
        }
    }
}