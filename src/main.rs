#![recursion_limit = "256"]

use dotenv::dotenv;
use matrix_sdk::{
    Client,
    config::SyncSettings,
    Room, RoomState,
    ruma::events::room::{
        member::StrippedRoomMemberEvent,
        message::{MessageType, OriginalSyncRoomMessageEvent},
    },
};
use tokio::runtime::Handle;
use tokio::time::{Duration, sleep};
use std::sync::OnceLock;
use std::sync::{LazyLock, Mutex};
use chrono::Local;
use cron_tab::Cron as Cron_tab;

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
use crate::services::schedule::ScheduleClient;

pub static MATRIX_CLIENT: OnceLock<Client> = OnceLock::new();
pub static CRON_SCHEDULER: LazyLock<Mutex<Cron_tab<Local>>> = LazyLock::new(|| {
    let mut scheduler = Cron_tab::new(Local);
        scheduler.start();
        Mutex::new(scheduler)
});

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
    recreate_all_cron();
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
    MATRIX_CLIENT.set(client.clone()).expect("Error, Matrix client is already initialised");
    client
        .matrix_auth()
        .login_username(username, password)
        .initial_device_display_name("getting started bot")
        .await?;

    println!("logged in as {username}");
    client.add_event_handler(message_listener);
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

async fn message_listener(ev: OriginalSyncRoomMessageEvent, room: Room) {
    if room.state() != RoomState::Joined {
        return;
    }
    let MessageType::Text(text_content) = ev.content.msgtype else {
        return;
    };

    let commande_line = text_content.body.trim().to_string();
    tokio::spawn(async move {
        controller_command(&commande_line, room).await
    });
}

fn recreate_all_cron() {
    let all_crons = models::crons::Cron::get_all();
    let mut scheduler = CRON_SCHEDULER.lock().unwrap();
    let handle = Handle::current();
    for mut cron in all_crons {
        let handle_clone = handle.clone();
        let room_id = cron.room.clone();
        let command = cron.command.clone();
        let job_id = match scheduler.add_fn(&cron.cron_expression, move || {
            let r_id = room_id.clone();
            let cmd = command.clone();
            handle_clone.spawn(async move {
                ScheduleClient::cron_job(&r_id, &cmd).await;
            });
        }) {
          Ok(id) => id.to_string(),
          Err(_) => {
              "impossible to create task".to_string()
          }
        };
        cron.job_id = job_id;
        models::crons::Cron::update_cron(&cron, cron.id);
    }
}
