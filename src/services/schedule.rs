use crate::models::crons::{NewCron, Cron as DbCron};
use tokio::runtime::Handle;
use std::fmt::Write;
use std::sync::LazyLock;
use matrix_sdk::ruma::RoomId;
use crate::MATRIX_CLIENT;
use regex::Regex;
use crate::CRON_SCHEDULER;


use super::controller::controller_command;

static DAY_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let d = "mon|tue|wed|thu|fri|sat|sun";
    let pattern = format!("^(?:{d})(?:,(?:{d}))*$|^(?:{d})-(?:{d})$");
    Regex::new(&pattern).unwrap()
});
static HOUR_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let pattern = format!("^([01][0-9]|2[0-3]):([0-5][0-9])$");
    Regex::new(&pattern).unwrap()
});


pub struct ScheduleClient;

impl ScheduleClient {
    fn is_valid_schedule_day(input: &str) -> bool {
        DAY_REGEX.is_match(input)
    }
    fn is_valide_schedule_hour(input: &str) -> bool {
        HOUR_REGEX.is_match(input)
    }
    fn get_hour_minute(input: &str) -> (String, String) {
        let mut hours = input.split(":");
        let hour = hours.next().unwrap_or("").to_lowercase();
        let minutes = hours.next().unwrap_or("").to_lowercase();
        (hour, minutes)
    }
    fn controller_create_cron(args: &str, room_id: &str) -> String {
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
        let final_cron = cron.unwrap_or_else(|| "mon-fri".to_string());
        let final_hour = hour.unwrap_or_else(|| "11:30".to_string());
        let final_job = match job {
            Some(j) => j,
            None => return format!("Error: The `-j` (job) flag is mandatory.\n{}", Self::schedule_help())
        };

        Self::create_cron(&final_hour, &final_cron, &final_job, room_id)
    }

    fn create_cron(complete_hour: &str, cron: &str, command: &str, room_id: &str) -> String {
        let room_id_closure = room_id.to_string();
        let command_closure = command.to_string();
        let handle = Handle::current();
        let (hour, minutes) = Self::get_hour_minute(&complete_hour);
        let cron_expression = format!("0 {} {} * * {} *", minutes, hour, cron);
        let mut scheduler = CRON_SCHEDULER.lock().unwrap();

        let job_id = match scheduler.add_fn(&cron_expression, move || {
            let r_id = room_id_closure.clone();
            let cmd = command_closure.clone();
            handle.spawn(async move {
                Self::cron_job(&r_id, &cmd).await;
            });
        }) {
          Ok(id) => id.to_string(),
          Err(_) => {
              return format!("Failed to create task, read the doc : {}", Self::schedule_help());
          }
        };
        drop(scheduler);
        let job_id_str = job_id;
        NewCron::create(&room_id, &cron_expression, &command, &job_id_str, &complete_hour);
        "Task has been scheduled successfully.".to_string()
    }

    pub async fn cron_job(room_id: &str, command: &str) {
        let Some(client) = MATRIX_CLIENT.get() else {
            eprintln!("Error: Matrix client not initialised when cron triggered.");
            return;
        };
        let Ok(parsed_room_id) = RoomId::parse(room_id) else {
            eprintln!("Invalid room id : {}", room_id);
            return;
        };
        if let Some(room) = client.get_room(&parsed_room_id) {
            controller_command(command, room).await;
        }
    }

    fn list_room_crons(room_id: &str) -> String {
        let room_crons = DbCron::get_by_room_id(room_id);
        if room_crons.is_empty() {
            return "There is no Task created in this room \n Create your first task with this command to schedule every day of the week a command : \n `!schedule -c mon-fri !menu hopper`".to_string()
        }
        let mut message = String::from("List of task of this room : \n");
        for cron in room_crons {
            if let Some(days) = cron.cron_expression.split_whitespace().nth(5) {
                let _ = writeln!(message, " - Cron name : **{}**", cron.name );
                let _ = writeln!(message, " Command : **{}**", cron.command);
                let _ = writeln!(message, " Day(s) : **{}**", days);
                let _ = writeln!(message, " Hour : **{}**", cron.hour);
            } else {
                let _ = writeln!(message, " - Command : **{}**", cron.command);
                let _ = writeln!(message, " Day(s) : **Undefined day(s)**");
                let _ = writeln!(message, " Hour : **{}**", cron.hour);
            }

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
    -l, --list          List all scheduled tasks in the current room.
    -h, --help          Print this help message.

OPTIONS FOR 'create':
    -d, --date <DAYS>   Specify the day(s) to execute the command, mon-fri by default.
    -j, --job <CMD>     The exact bot command to run.
    -h, --hour <HOUR>   Specify the hour to execute the command, 11:30 by default.

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

    pub async fn controller_schedule(args: &str, room_id: &str) -> String {
        let mut iter = args.split_ascii_whitespace();
        match iter.next() {
            Some("create") => {
                let remaining_args = args.trim_start_matches("create").trim();
                if !remaining_args.is_empty() {
                    Self::controller_create_cron(remaining_args, room_id)
                } else {
                    Self::schedule_help().to_string()
                }
            }
            Some("-l" | "--list") => Self::list_room_crons(&room_id),
            _ => Self::schedule_help().to_string(),
        }
    }
}
