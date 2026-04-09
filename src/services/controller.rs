use matrix_sdk::{
    Room,
    ruma::{
        events::room::{
        message::RoomMessageEventContent,
        }
    }
};
use std::sync::Arc;

use super::list::list_restaurant;
use super::menu::get_menu;
use super::yum::get_restaurant;
use super::oslf::get_fries;
use super::help::get_help;
use super::schedule::ScheduleClient;
use crate::AppState;

pub fn controller_command<'a>(commande_line: &'a str, room: Room, state: &'a Arc<AppState>) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'a>> {
    Box::pin(async move {
    let (commande, args) = commande_line
        .split_once(' ')
        .unwrap_or((commande_line, ""));

    let response: String = match commande {
        "!schedule" => {
            ScheduleClient::controller_schedule(&args, &room.room_id().to_string(), state).await
        }
        "!yum" => {
            get_restaurant(&args.trim(), &state.cache, &state.api).await
        }
        "!menu" => {
            get_menu(&args.trim(), &state.cache, &state.api).await
        }
        "!oslf" => {
            get_fries(&args.trim(), &state.cache, &state.api).await
        }
        "!list" => {
            list_restaurant(&args.trim(), &state.cache, &state.api).await
        }
        "!help" => {
            get_help(&state.config.bot_repo, &state.config.bot_version)
        }
        _ => {
            return;
        }
    };
    if let Err(e) = room.send(set_message(&response)).await {
        log::warn!("[{}] Failed to send message to room {} : {}",
        commande,
        room.room_id(),
        e
        )
        }
    })
}

fn set_message(message: &str) -> RoomMessageEventContent {
    return RoomMessageEventContent::text_markdown(message);
}
