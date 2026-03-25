use crate::models::crons::{NewCron, Cron as DbCron};
use tokio::runtime::Handle;
use std::fmt::Write;
use chrono::Local;
use cron_tab::Cron;
use matrix_sdk::ruma::RoomId;
use crate::MATRIX_CLIENT;
use regex::Regex;


use super::controller::controller_command;

pub struct ScheduleClient;

impl ScheduleClient {
    fn is_valid_schedule_day(input: &str) -> bool {
        let d = "mon|tue|wed|thu|fri";
        let pattern = format!("^(?:{d})(?:,(?:{d}))*$|^(?:{d})-(?:{d})$");

        let re = Regex::new(&pattern).unwrap();
        re.is_match(input)
    }

    fn create_cron(args: &str, room_id: &str) -> String {
        let (cron, command) = Self::get_cron_command(&args);

        if !Self::is_valid_schedule_day(&cron) {
            return "Error: Invalid day pattern. Please use a 3-letter day (e.g., mon), a list (mon,wed), or a range (mon-fri), sat and sun are not accepted.\n Run !schedule --help or -h for more help".to_string()
        }
        let mut scheduler = Cron::new(Local);
        let room_id_closure = room_id.to_string();
        let command_closure = command.clone();
        let handle = Handle::current();
        let cron_expression = format!("0 30 11 * * {} *", cron);
        let job_id = scheduler.add_fn(&cron_expression, move || {
            let r_id = room_id_closure.clone();
            let cmd = command_closure.clone();
            handle.spawn(async move {
                Self::cron_job(&r_id, &cmd).await;
            });
        }).expect("Failed to create new cron");
        let new_cron = NewCron {
            room: room_id,
            cron_expression: &cron_expression,
            command: args,
            job_id: &job_id.to_string(),
        };
        new_cron.create();
        scheduler.start();
        return "Task has been scheduled with successed".to_string();
    }

    async fn cron_job(room_id: &str, command: &str) {
        let client = MATRIX_CLIENT.get().expect("Error, matrix client not initialised");

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
        let room_crons = DbCron::get_by_room_id(room_id);
        let mut message = String::from("List of task of this room : \n");
        for cron in room_crons {
            let mut commands = cron.command.split_whitespace();
            let days = commands.next().unwrap_or("").to_lowercase();
            let command: Vec<&str> = commands.by_ref().collect();
            let _ = writeln!(message, " - Command : **{}**", command.join(" "));
            let _ = writeln!(message, " Day(s) : **{}**", days);
        }
        message
    }

    fn schedule_help() -> String {
        return format!("\
## Command Overview: `!schedule`

The `!schedule` command allows you to automate bot commands to execute in the current room at exactly **11:30 AM**. By using flexible, cron-style formatting for the day parameter, you can create highly specific recurring schedules without needing to set up multiple identical commands.

## Syntax

`!schedule [day_pattern] [<command>]`

## Parameters

* **`[day_pattern]`**: The day or combination of days you want the command to execute. It accepts single days, lists, and ranges.
* **`[<command>]`**: The exact bot command (including any of its own parameters) that you want to automate.

## Advanced Scheduling Options

The `[day_pattern]` parameter uses a smart scheduling logic. You can use commas and hyphens to group days together, just like traditional cron jobs:

* **Single Day**: Use the standard 3-letter abbreviation for a single day.
  * *Syntax:* `mon`, `tue`, `wed`, `thu`, `fri`, `sat`, `sun`
* **Specific List (Commas)**: Use a comma to select multiple, non-consecutive days. Do not include spaces between the days.
  * *Syntax:* `mon,wed,fri` (Triggers on Mondays, Wednesdays, and Fridays)
* **Day Range (Hyphens)**: Use a hyphen to select a continuous block of days.
  * *Syntax:* `mon-fri` (Triggers every weekday from Monday through Friday)

## Practical Examples

### 1. The Weekday Routine (Range)
> `!schedule mon-fri !menu hopper`
> Automatically triggers the `!menu hopper` command in the room every weekday (Monday through Friday) at 11:30 AM.

### 2. The Custom Selection (List)
> `!schedule mon,tue,fri !yum pizza`
> Sends the yum command to the room only on Mondays, Tuesdays, and Fridays at 11:30 AM.

### 3. The Single Weekly Event (Single Day)
> `!schedule thu !menu hopper`
> Triggers the menu command in the room every Thursday at 11:30 AM.
            ");
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
                _ => {
                    response = Self::schedule_help();
                    break;
                }
            }
        }
        response
    }
}
