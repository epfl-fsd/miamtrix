use crate::models::crons::{NewCron, Cron as DbCron};
use std::fmt::Write;
use chrono::Local;
use cron_tab::Cron;
use matrix_sdk::ruma::RoomId;
use crate::MATRIX_CLIENT;

use super::controller::controller_command;

pub struct ScheduleClient;

impl ScheduleClient {
    fn create_cron(args: &str, room_id: &str) -> String {
        let (cron, command) = Self::get_cron_command(&args);
        let mut scheduler = Cron::new(Local);

        let room_id_closure = room_id.to_string();
        let command_closure = command.clone();
        let cron_expression = format!("35 16 * * * {} *", cron);
        let job_id = scheduler.add_fn(&cron_expression, move || {
            let r_id = room_id_closure.clone();
            let cmd = command_closure.clone();
            tokio::spawn(async move {
                Self::cron_job(&r_id, &cmd).await;
            });
        }).expect("Failed to create new cron");
        let new_cron = NewCron {
            room: room_id,
            cron_expression: "30 11 * * mon-fri",
            command: args,
            job_id: &job_id.to_string(),
        };
        let cron = new_cron.create();
        return "Create Cron".to_string();
    }

    async fn cron_job(room_id: &str, command: &str) {
        let client = MATRIX_CLIENT.get().expect("Error, matrix client not initilised");

        let Ok(parsed_room_id) = RoomId::parse(room_id) else {
            eprintln!("Invalid room id : {}", room_id);
            return;
        };
        if let Some(room) = client.get_room(&parsed_room_id) {
            controller_command(command, room).await;
        }
    }

    fn get_cron_command(params: &str) -> (String, String) {
        if params.is_empty() {
            return ("".to_string(), "".to_string());
        }
        let mut parts = params.splitn(2, ' ');
        let cron = parts.next().unwrap_or("").to_lowercase();
        let command = parts.next().unwrap_or("").to_lowercase();
        (cron, command)
    }

    fn list_room_crons(room_id: &str) -> String {
        let all_crons = DbCron::get_all();
        let mut message = String::from("All crons : \n");
        for cron in all_crons {
            let _ = writeln!(message, " - **{}**", cron.command);
        }
        message
    }

    pub async fn controller_schedule(args: &str, room_id: &str) -> String {
        let mut iter = args.split_ascii_whitespace();
        let mut response = "Explanation Schedule".to_string();
        while let Some(token) = iter.next() {
            match token {
                "-c" => {
                    let parts: Vec<&str> = iter.by_ref().collect();
                    if !parts.is_empty() {
                        response = Self::create_cron(&parts.join(" "), room_id);
                    } else {
                        response = "Create but no argument".to_string()
                    }
                    break;
                }
                "-l" => {
                    response = Self::list_room_crons(&room_id);
                    break;
                }
                _ => {}
            }
        }
        response
    }
}
