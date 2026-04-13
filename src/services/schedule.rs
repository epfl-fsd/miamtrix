use std::sync::Arc;
use std::fmt::Write;
use std::sync::LazyLock;
use matrix_sdk::ruma::RoomId;
use regex::Regex;
use tokio_cron_scheduler::Job;
use chrono_tz::Europe::Zurich;

use crate::models::crons::{NewCron, Cron as DbCron};
use crate::AppState;
use super::controller::controller_command;

static DAY_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let d = "mon|tue|wed|thu|fri|sat|sun";
    Regex::new(&format!("^(?:{d})(?:,(?:{d}))*$|^(?:{d})-(?:{d})$")).unwrap()
});
static HOUR_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("^([01][0-9]|2[0-3]):([0-5][0-9])$").unwrap()
});


pub struct ScheduleClient;

impl ScheduleClient {
    fn is_valid_schedule_day(input: &str) -> bool {
        DAY_REGEX.is_match(input)
    }
    fn is_valide_schedule_hour(input: &str) -> bool {
        HOUR_REGEX.is_match(input)
    }
    fn get_hour_minute(input: &str) -> (&str, &str) {
        let mut parts = input.splitn(2, ":");
        let hour = parts.next().unwrap_or("");
        let minutes = parts.next().unwrap_or("");
        (hour, minutes)
    }
    async fn controller_create_cron(args: &str, room_id: &str, state: &Arc<AppState>) -> String {
        let mut cron = None;
        let mut job = None;
        let mut hour = None;

        let mut iter = args.split_whitespace().peekable();

        while let Some(word) = iter.next() {
            match word {
                "-d" | "--date" => {
                    if let Some(d) = iter.next() {
                        if Self::is_valid_schedule_day(d) {
                            cron = Some(d.to_string());
                        } else {
                            return "Error: Invalid day pattern. Please use a 3-letter day (e.g., mon), a list (mon,wed), or a range (mon-fri).\n Run !schedule --help or -h for more help".to_string()
                        }

                    } else {
                        return "Error : Missing argument for `-d`. Example: `-d mon-fri`".to_string();
                    }
                }
                "-j" | "--job" => {
                    let mut job_parts = Vec::new();

                    while let Some(&next_word) = iter.peek() {
                        if next_word == "-d" || next_word == "--date" || next_word == "-h" || next_word == "--hour" {
                            break;
                        }
                        job_parts.push(iter.next().unwrap());

                    }
                    if job_parts.is_empty() {
                        return format!("Error: missing argument after `-j | --job`");
                    }
                    job = Some(job_parts.join(" "));
                }
                "-h" | "--hour" => {
                    if let Some(h) = iter.next() {
                        if Self::is_valide_schedule_hour(h) {
                            hour = Some(h.to_string());
                        } else {
                            return "Error: Invalid hour format. Please use this format HH:mm (e.g., 11:45).\n Run !schedule --help for more help".to_string()
                        }
                    } else {
                        return "Error : Missing argument for `-h` | `--hour`. Example: `-h 11:30`".to_string();
                    }
                }
                _ => {
                    return format!("Error: unknown argument: `{}` \n {}", word,  Self::schedule_help());
                }
            }
        }
        let final_days = cron.unwrap_or_else(|| "mon-fri".to_string());
        let final_hour = hour.unwrap_or_else(|| "11:30".to_string());
        let final_job = match job {
            Some(j) => j,
            None => return format!("Error: The `-j` (job) flag is mandatory.\n{}", Self::schedule_help())
        };

        Self::create_cron(&final_hour, &final_days, &final_job, room_id, state).await
    }

    async fn create_cron(complete_hour: &str, days: &str, command: &str, room_id: &str, state: &Arc<AppState>) -> String {
        let (hour, minutes) = Self::get_hour_minute(&complete_hour);
        let cron_expression = format!("0 {} {} * * {} *", minutes, hour, days);

        let state_clone = Arc::clone(state);
        let room_id_owned = room_id.to_string();
        let command_owned = command.to_string();


        let job = match Job::new_async_tz(cron_expression.as_str(), Zurich, move |_uuid, _lock| {
            let state = Arc::clone(&state_clone);
            let r_id = room_id_owned.clone();
            let cmd = command_owned.clone();
            Box::pin(async move {
                Self::cron_job(&r_id, &cmd, &state).await;
            })
        }) {
          Ok(j) => j,
          Err(_) => {
              return format!("Failed to create task, read the doc : {}", Self::schedule_help());
          }
        };
        let job_id = match state.scheduler.add(job).await {
            Ok(id) => id.to_string(),
            Err(e) => {
                log::warn!("Failed to schedule job: {}", e);
                return format!("Failed to schedule job");
            }
        };

        if let Err(e) = NewCron::create(
            &state.db,
            room_id,
            &cron_expression,
            command,
            &job_id,
            complete_hour,
        ).await {
            log::warn!("Job scheduled but failed to persist: {}", e);
            return format!("Job scheduled but failed to persist");
        }
        log::info!("New task created for this room : {}", room_id);
        "Task has been scheduled successfully.".to_string()
    }

    pub async fn cron_job(room_id: &str, command: &str, state: &Arc<AppState>) {
        let Ok(parsed_room_id) = RoomId::parse(room_id) else {
            log::info!("Invalid room id: {}", room_id);
            return;
        };

        if let Some(room) = state.matrix_client.get_room(&parsed_room_id) {
            controller_command(command, room, state).await;
        }
    }

    async fn delete_cron(args: &str, room_id: &str, state: &Arc<AppState>) -> String {
        let mut task: Option<String> = None;
        let mut iter = args.split_whitespace().peekable();

        while let Some(word) = iter.next() {
            match word {
                "-n" | "--name" => {
                    let mut name_parts = Vec::new();
                    while let Some(&_next_word) = iter.peek() {
                        name_parts.push(iter.next().unwrap());
                    }
                    if !name_parts.is_empty() {
                        task = Some(name_parts.join(" "));
                    }
                }
                _ => {
                    continue;
                }
            }
        }
        let Some(name) = task else {
            return "Failed to delete cron, please specify the name of the task. Example: `!schedule delete -n task_name`".to_string();
        };

        let room_crons = DbCron::get_by_room_id(&state.db, room_id).await;
        let cron_to_delete = room_crons.into_iter().find(|c| c.name == name);

        let Some(cron) = cron_to_delete else {
            return format!("Error : Unknown task '{}'.", name);
        };

        if let Ok(uuid) = uuid::Uuid::parse_str(&cron.job_id) {
            if let Err(e) = state.scheduler.remove(&uuid).await {
                log::error!("Failed to remove task : {} from room : {}, {}", &name, &room_id, e);
                return "Failed to remove task, please try again".to_string();
            } else {
                log::info!("Task {} remove successfully", uuid);
            }
        } else {
            log::error!("Invalide Id in db : {}", cron.job_id);
            return "Failed to remove task, please try again".to_string();
        }

        let db_selected = DbCron::delete_cron(&state.db, room_id, &name).await;
        if db_selected {
            format!("Task : `{}` has been removed successfully", name)
        } else {
            "Failed to remove task, please try again".to_string()
        }

    }

    async fn list_room_crons(room_id: &str, state: &Arc<AppState>) -> String {
        let room_crons = DbCron::get_by_room_id(&state.db, room_id).await;

        if room_crons.is_empty() {
            return "There is no Task created in this room \n Create your first task with this command to schedule every day of the week a command : \n `!schedule -c mon-fri !menu hopper`".to_string()
        }
        let mut message = String::with_capacity(room_crons.len() * 100);
        message.push_str("List of tasks in this room:\n");

        for cron in room_crons {
            let days = cron.cron_expression
                .split_whitespace()
                .nth(5)
                .unwrap_or("Undefined");
            let _ = writeln!(message, " - name : **{}** \n task : `{}` \n day(s) : {} \n hour : {} \n id : {} \n", cron.name, cron.command, days, cron.hour, cron.job_id);
        }
        message
    }

    fn schedule_help() -> &'static str {
        "\
## Command Overview: `!schedule`

Automate bot commands to execute in the current room exactly at 11:30 AM.

USAGE:
    !schedule <SUBCOMMAND> [OPTIONS]

SUBCOMMANDS:
    create              Create a new scheduled task.
    delete              Delete a scheduled task.
    -l, --list          List all scheduled tasks in the current room.
    --help              Print this help message.

OPTIONS FOR 'create':
    -d, --date <DAYS>   Specify the day(s) to execute the command, mon-fri by default.
    -j, --job <CMD>     The exact bot command to run.
    -h, --hour <HOUR>   Specify the hour to execute the command, 11:30 by default.

OPTIONS FOR 'delete':
    -n, --name <NAME>   Specify the name of the cron you want delete.

DAY PATTERNS:
    You can use cron-style formatting for the <DAYS> parameter:
    * Single Day : mon, tue, wed, thu, fri, sat, sun
    * List       : mon,wed,fri (Comma-separated, NO spaces)
    * Range      : mon-fri     (Hyphen-separated)

EXAMPLES:
    1. The Weekday Routine (Range)
       `!schedule create -d mon-fri -j !menu hopper`

    2. The Custom Selection (List)
       `!schedule create -d mon,tue,fri -j !yum pizza`

    3. The Single Weekly Event (Single Day)
       `!schedule create -d thu -j !menu hopper`

    4. List all tasks of this room
       `!schedule --list`
            "
    }

    pub async fn controller_schedule(args: &str, room_id: &str, state: &Arc<AppState>) -> String {
        let mut iter = args.split_ascii_whitespace();
        match iter.next() {
            Some("create") => {
                let remaining_args = args.trim_start_matches("create").trim();
                if !remaining_args.is_empty() {
                    Self::controller_create_cron(remaining_args, room_id, state).await
                } else {
                    Self::schedule_help().to_string()
                }
            }
            Some("delete") => {
                let remaining_args = args.trim_start_matches("delete").trim();
                if !remaining_args.is_empty() {
                    Self::delete_cron(remaining_args, room_id, state).await
                } else {
                    Self::schedule_help().to_string()
                }
            }
            Some("-l" | "--list") => Self::list_room_crons(&room_id, state).await,
            _ => Self::schedule_help().to_string(),
        }
    }
}
