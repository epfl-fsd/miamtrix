use matrix_sdk::{
    Room, RoomState,
    ruma::events::room::{
        message::{OriginalSyncRoomMessageEvent, MessageType, RoomMessageEventContent},
    },
};
use super::menu::get_menu;
use super::miam::get_restaurant;
use super::oslf::get_fries;

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
        "/miam" => {
            let restaurant = get_restaurant("végé");
            room.send(set_message(restaurant)).await.unwrap();
        }
        "/menu" => {
            if args.is_empty() {
                room.send(set_message("Il faut préciser un restaurant dans la commande")).await.unwrap();
            } else {
                let menu = get_menu(&restaurant_filter.trim()).await;
                room.send(set_message(&menu)).await.unwrap();
            }
        }
        "/oslf" => {
            let fries = get_fries();
            room.send(set_message(&fries)).await.unwrap();
        }
        "/help" => {
            let help_message = "Commande disponible : `/miam`, `/menu`, `/oslf`, `/help`";
            room.send(set_message(help_message)).await.unwrap();
        }
        _ => {
            println!("Message ignoré : {}", commande_line)
        }
    }
}

fn set_message(message: &str) -> RoomMessageEventContent {
    return RoomMessageEventContent::text_markdown(message);
}
