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
    pub bot_version: String,
    pub bot_repo: String,
    pub db_url: String,
}

pub static CONFIG: OnceLock<AppConfig> = OnceLock::new();

impl AppConfig {
    pub fn load_env() -> Self {
        AppConfig {
            api_uri: env::var("URI_API").expect("Missing variable"),
            api_username: env::var("API_USERNAME").expect("Missing variable"),
            api_password: env::var("API_PASSWORD").expect("Missing variable"),
            bot_username: env::var("USERNAME_BOT").expect("Missing variable"),
            bot_password: env::var("PASSWORD_BOT").expect("Missing variable"),
            url_server_matrix: env::var("URL_SERVER_MATRIX").expect("Missing variable"),
            bot_version: env!("CARGO_PKG_VERSION").to_string(),
            bot_repo: env!("CARGO_PKG_REPOSITORY").to_string(),
            db_url: env::var("DATABASE_URL").expect("Missing variable"),
        }
    }
}
