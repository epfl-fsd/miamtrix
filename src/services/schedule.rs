use crate::models::crons::{NewCron, Cron as DbCron};
use tokio::runtime::Handle;
use std::fmt::Write;
use std::sync::{LazyLock, Mutex};
use chrono::Local;
use cron_tab::Cron;
use matrix_sdk::ruma::RoomId;
use crate::MATRIX_CLIENT;
use regex::Regex;


use super::controller::controller_command;

static DAY_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let d = "mon|tue|wed|thu|fri|sat|sun";
    let pattern = format!("^(?:{d})(?:,(?:{d}))*$|^(?:{d})-(?:{d})$");
    Regex::new(&pattern).unwrap()
});

static CRON_SCHEDULER: LazyLock<Mutex<Cron<Local>>> = LazyLock::new(|| {
    let mut scheduler = Cron::new(Local);
        scheduler.start();
        Mutex::new(scheduler)
});

pub struct ScheduleClient;

impl ScheduleClient {
    fn is_valid_schedule_day(input: &str) -> bool {
        DAY_REGEX.is_match(input)
    }
    fn controller_create_cron(args: &str, room_id: &str) -> String {
        //get cron si cron et get command obligatoire
        let mut cron = None;
        let mut job = None;

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
                        if next_word == "-d" || next_word == "--date" {
                            break;
                        }
                        job_parts.push(iter.next().unwrap());

                    }
                    if job_parts.is_empty() {
                        return format!("Error: missing command after `-j`");
                    }
                    job = Some(job_parts.join(" "));
                }
                _ => {
                    return format!("Error: unknown argument: `{}` \n {}", word,  Self::schedule_help());
                }
            }
        }
        let final_cron = cron.unwrap_or_else(|| "mon-fri".to_string());

        let final_job = match job {
            Some(j) => j,
            None => return format!("Error: The `-j` (job) flag is mandatory.\n{}", Self::schedule_help())
        };

        Self::create_cron(&final_cron, &final_job, room_id)
    }

    fn create_cron(cron: &str, command: &str, room_id: &str) -> String {
        let room_id_closure = room_id.to_string();
        let command_closure = command.to_string();
        let handle = Handle::current();
        let cron_expression = format!("0 30 11 * * {} *", cron);
        let mut scheduler = CRON_SCHEDULER.lock().unwrap();

        let job_id = match scheduler.add_fn(&cron_expression, move || {
            let r_id = room_id_closure.clone();
            let cmd = command_closure.clone();
            handle.spawn(async move {
                Self::cron_job(&r_id, &cmd).await;
            });
        }) {
          Ok(id) => id,
          Err(_) => {
              return format!("Failed to create task, read the doc : {}", Self::schedule_help());
          }
        };
        drop(scheduler);
        let job_id_str = job_id.to_string();
        let new_cron = NewCron {
            room: room_id,
            cron_expression: &cron_expression,
            command: command,
            job_id: &job_id_str,
        };
        new_cron.create();
        "Task has been scheduled successfully.".to_string()
    }

    async fn cron_job(room_id: &str, command: &str) {
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
            let commands = cron.command;
            if let Some(days) = cron.cron_expression.split_whitespace().nth(5) {
                let _ = writeln!(message, " - Command : **{}**", commands);
                let _ = writeln!(message, " Day(s) : **{}**", days);
            } else {
                let _ = writeln!(message, " - Command : **{}**", commands);
                let _ = writeln!(message, " Day(s) : **Undefined day(s)**");
            }

        }
        message
    }

    fn schedule_help() -> &'static str {
        "\
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

## Examples

### 1. The Weekday Routine (Range)
> `!schedule -c mon-fri !menu hopper`
> Automatically triggers the `!menu hopper` command in the room every weekday (Monday through Friday) at 11:30 AM.

### 2. The Custom Selection (List)
> `!schedule -c mon,tue,fri !yum pizza`
> Sends the yum command to the room only on Mondays, Tuesdays, and Fridays at 11:30 AM.

### 3. The Single Weekly Event (Single Day)
> `!schedule -c thu !menu hopper`
> Triggers the menu command in the room every Thursday at 11:30 AM.

### 4. List all tasks of this room
> `!schedule -l`
> List all tasks of this room
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
