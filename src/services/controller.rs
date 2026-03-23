use matrix_sdk::{
    Room, RoomState,
    ruma::events::room::{
        message::{OriginalSyncRoomMessageEvent, MessageType, RoomMessageEventContent},
    },
};
use super::menu::get_menu;
use super::yum::get_restaurant;
use super::oslf::get_fries;
use super::help::get_help;

pub async fn controller_command(ev: OriginalSyncRoomMessageEvent, room: Room) {
    if room.state() != RoomState::Joined {
        return;
    }
    let MessageType::Text(text_content) = ev.content.msgtype else {
        return;
    };
    let commande_line = text_content.body.trim();

    let (commande, args) = match commande_line.split_once(' ') {
        Some((cmd, reste)) => (cmd, reste),
        None => (commande_line, ""),
    };

    match commande {
        "!yum" => {
            let restaurant = get_restaurant(&args.trim()).await;
            room.send(set_message(&restaurant)).await.unwrap();
        }
        "!menu" => {
            let menu = get_menu(&args.trim()).await;
            room.send(set_message(&menu)).await.unwrap();
        }
        "!oslf" => {
            let fries = get_fries().await;
            room.send(set_message(&fries)).await.unwrap();
        }
        "!help" => {
            let help_message = get_help();
            room.send(set_message(&help_message)).await.unwrap();
        }
        _ => {
            println!("Message ignoré : {}", commande_line)
        }
    }
}

fn set_message(message: &str) -> RoomMessageEventContent {
    return RoomMessageEventContent::text_markdown(message);
}
