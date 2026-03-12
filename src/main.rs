use dotenv::dotenv;
use matrix_sdk::{
    Client,
    config::SyncSettings,
    ruma::events::room::{
        message::OriginalSyncRoomMessageEvent,
    },
};
mod config;
mod services;
mod utils;

use crate::utils::api::ApiClient;
use crate::services::controller::controller_command;
use crate::config::{AppConfig, CONFIG};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let config_app = AppConfig::load_env();

    let url = config_app.url_server_matrix.clone();
    let user = config_app.bot_username.clone();
    let pass = config_app.bot_password.clone();
    CONFIG.set(config_app).expect("Config already init");
    ApiClient::init();

    login_and_sync(url, &user, &pass).await?;

    Ok(())
}



async fn login_and_sync(
    homeserver_url: String,
    username: &str,
    password: &str,
) -> anyhow::Result<()> {
    let client = Client::builder()
        .homeserver_url(homeserver_url)
        .build()
        .await?;

    client
        .matrix_auth()
        .login_username(username, password)
        .initial_device_display_name("getting started bot")
        .await?;

    println!("logged in as {username}");
    client.add_event_handler(controller_command);
    let sync_token = client.sync_once(SyncSettings::default()).await.unwrap().next_batch;

    let settings = SyncSettings::default().token(sync_token);
    client.sync(settings).await?;

    Ok(())
}
