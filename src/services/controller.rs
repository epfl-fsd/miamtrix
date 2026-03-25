use matrix_sdk::{
    Room,
    ruma::{
        events::room::{
        message::RoomMessageEventContent,
        }
    }
};

use super::list::list_restaurant;
use super::menu::get_menu;
use super::yum::get_restaurant;
use super::oslf::get_fries;
use super::help::get_help;
use super::schedule::ScheduleClient;

pub async fn controller_command(commande_line: &str, room: Room) {
    let (commande, args) = match commande_line.split_once(' ') {
        Some((cmd, reste)) => (cmd, reste),
        None => (commande_line, ""),
    };

    match commande {
        "!schedule" => {
            let response = ScheduleClient::controller_schedule(&args, &room.room_id().to_string()).await;
            room.send(set_message(&response)).await.unwrap();
        }
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
        "!list" => {
            let list = list_restaurant().await;
            room.send(set_message(&list)).await.unwrap();
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
