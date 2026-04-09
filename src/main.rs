#![recursion_limit = "256"]

use std::sync::Arc;
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
use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};
use tokio_cron_scheduler::JobScheduler;
use chrono_tz::Europe::Zurich;
use log;

mod config;
mod services;
mod utils;
mod models;
mod schema;
mod db;

use crate::{config::AppConfig, utils::logs};
use crate::db::{create_pool, run_migrations, DbPool};
use crate::utils::api::ApiClient;
use crate::utils::cache::{create_cache, SharedCache};
use crate::services::controller::controller_command;
use crate::services::schedule::ScheduleClient;
use crate::models::crons::Cron;

pub struct AppState {
    pub db: DbPool,
    pub api: Arc<ApiClient>,
    pub cache: SharedCache,
    pub matrix_client: Arc<Client>,
    pub config: Arc<AppConfig>,
    pub scheduler: JobScheduler,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let _log_guards = logs::init_logging();
    log::info!("Starting application");
    dotenv().ok();

    log::info!("Load config");
    let config = Arc::new(AppConfig::load_env());

    run_migrations(&config.db_url);

    log::info!("Create db pool");
    let db_pool = create_pool(&config.db_url);

    log::info!("Init ApiClient");
    let api = Arc::new(ApiClient::new(
        config.api_uri.clone(),
        config.api_username.clone(),
        config.api_password.clone(),
    ));

    let cache = create_cache();

    log::info!("Init Job Scheduler");
    let scheduler = JobScheduler::new().await?;

    log::info!("Init Matrix client");
    let matrix_client = Arc::new(
        Client::builder()
            .homeserver_url(&config.url_server_matrix)
            .build()
            .await?
    );

    log::info!("Authentication in matrix");
    matrix_client
        .matrix_auth()
        .login_username(&config.bot_username, &config.bot_password)
        .initial_device_display_name("Matrix-bot")
        .await?;

    log::info!("logged in as {}", config.bot_username);

    let state = Arc::new(AppState {
        db: db_pool,
        api,
        cache,
        matrix_client: Arc::clone(&matrix_client),
        config,
        scheduler
    });
    recreate_all_cron(&state).await;
    state.scheduler.start().await?;
    login_and_sync(state).await?;

    Ok(())
}



async fn login_and_sync(state: Arc<AppState>) -> anyhow::Result<()> {

    let (tx, mut rx) = mpsc::channel::<(String, Room)>(256);
    let tx_clone = tx.clone();

    state.matrix_client.add_event_handler(move |ev: OriginalSyncRoomMessageEvent, room: Room| {
        let tx = tx_clone.clone();
        async move {
            if room.state() != RoomState::Joined {
                return;
            }
            let MessageType::Text(text_content) = ev.content.msgtype else {
                return;
            };
            let cmd = text_content.body.trim().to_string();
            if !cmd.starts_with('!') {
                return;
            }
            if tx.try_send((cmd, room)).is_err() {
                eprintln!("Message queue full, dropping message");
            }
        }
    });
    state.matrix_client.add_event_handler(auto_accept_invites);
    let state_dispatcher = Arc::clone(&state);
        tokio::spawn(async move {
            while let Some((cmd, room)) = rx.recv().await {
                let state = Arc::clone(&state_dispatcher);
                tokio::spawn(async move {
                    log::info!("Command : {}, From : {}", &cmd, &room.room_id());
                    controller_command(&cmd, room, &state).await;
                });
            }
        });

        let sync_token = state.matrix_client
            .sync_once(SyncSettings::default())
            .await?
            .next_batch;

        let settings = SyncSettings::default().token(sync_token);
        state.matrix_client.sync(settings).await?;

        Ok(())
}

async fn auto_accept_invites(
    room_member: StrippedRoomMemberEvent,
    client: Client,
    room: Room,
) {
    if room_member.state_key != client.user_id().unwrap() {
        return;
    }
    tokio::spawn(async move {
        let mut delay = 2u64;
        while let Err(err) = room.join().await {
            log::warn!("Failed to join {} ({err}), retry in {delay}s", room.room_id());
            sleep(Duration::from_secs(delay)).await;
            delay = (delay * 2).min(3600);
        }
        log::info!("Joined room {}", room.room_id());
    });
}

async fn recreate_all_cron(state: &Arc<AppState>) {
    log::info!("Recreate all crons");
    let all_crons = Cron::get_all(&state.db).await;
    for cron in &all_crons {
        let state_clone = Arc::clone(state);
        let room_id = cron.room.clone();
        let command = cron.command.clone();

        let job = match tokio_cron_scheduler::Job::new_async_tz(
            cron.cron_expression.as_str(),
            Zurich,
            move |_uuid, _lock| {
                let state = Arc::clone(&state_clone);
                let r_id = room_id.clone();
                let cmd = command.clone();
                Box::pin(async move {
                    ScheduleClient::cron_job(&r_id, &cmd, &state).await;
                })
            },
        ) {
            Ok(j) => j,
            Err(e) => {
                log::error!("Failed to recreate cron '{}': {}", cron.command, e);
                continue;
            }
        };

        if let Err(e) = state.scheduler.add(job).await {
            log::error!("Failed to add cron to scheduler: {}", e);
        }
    }

    log::info!("Recreated {} cron jobs", all_crons.len());
}
