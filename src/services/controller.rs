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

    println!("Nouveau message reçu : {:?}", text_content.body);

    if text_content.body.contains("/miam") {
        let restaurant = get_restaurant("végé");
        room.send(set_message(restaurant)).await.unwrap();
    } else if text_content.body.contains("/menu") {
        let menu = get_menu("Hopper");
        room.send(set_message(menu)).await.unwrap();
    } else if text_content.body.contains("/oslf") {
        let fries = get_fries();
        room.send(set_message(&fries)).await.unwrap();
    } else {
        println!("Message non compris : {:?}", text_content.body);
    }
}

fn set_message(message: &str) -> RoomMessageEventContent {
    return RoomMessageEventContent::text_plain(message);
}