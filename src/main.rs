#![recursion_limit = "256"]

use dotenv::dotenv;
use matrix_sdk::{
    Client,
    config::SyncSettings,
    Room,
    ruma::events::room::{
        member::StrippedRoomMemberEvent,
    },
};
use tokio::time::{Duration, sleep};

mod config;
mod services;
mod utils;
mod models;
mod schema;
mod db;

use crate::db::DbClient;
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
    let db_url = config_app.db_url.clone();
    CONFIG.set(config_app).expect("Config already init");
    ApiClient::init();
    DbClient::init(&db_url);

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
    //Add auto join function to handler
    client.add_event_handler(auto_accept_invites);
    let sync_token = client.sync_once(SyncSettings::default()).await.unwrap().next_batch;

    let settings = SyncSettings::default().token(sync_token);
    client.sync(settings).await?;

    Ok(())
}

//Auto join function to accepte invitation
async fn auto_accept_invites(
    room_member: StrippedRoomMemberEvent,
    client: Client,
    room: Room,
) {
    if room_member.state_key != client.user_id().unwrap() {
            return;
        }

    tokio::spawn(async move {
        println!("Autojoining room {}", room.room_id());
        let mut delay = 2;

        while let Err(err) = room.join().await {
            eprintln!("Failed to join room {} ({err:?}), retrying in {delay}s", room.room_id());

            sleep(Duration::from_secs(delay)).await;
            delay *= 2;

            if delay > 3600 {
                eprintln!("Can't join room {} ({err:?})", room.room_id());
                break;
            }
        }
        println!("Successfully joined room {}", room.room_id());
    });
}
