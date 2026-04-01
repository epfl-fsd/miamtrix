use crate::config::CONFIG;

pub fn get_help() -> String {
    let config = CONFIG.get().expect("Failed to load config");

    let help_message = format!("\
    Available commands: `!menu`, `!yum`, `!oslf`, `!help`, `!schedule`
    `!schedule` create [-d | --day <days>] [-j | --job <command>] [-h | --hour <hour>]      Schedules a command to run automatically at 11:30 AM on the specified day(s). Example: `!schedule create -d mon -j !menu hopper` runs the command every Monday. Use `!schedule -h` for full details.
    `!menu` [restaurant] [filter]                       Get a restaurant's menu with an optional filter (e.g., pizza)
    `!yum`  [-s | --search <keyword>] [-a | --allergen <allergen>] [-c | --city <city>]      Search all EPFL menus for a food (e.g., pizza), Use `!` to exclude (e.g., !fish), Use `-a` to specify an alergen (e.g., -a gluten).
    `!oslf` [-c | --city <city>]                        List locations of french fries across EPFL's campus.
    `!list` [-c | --city <city>]                        List all restaurants across EPFL's campus.
    `!help`                                             Get help with commands.
    version: `{}`, [source code]({}).", config.bot_version, config.bot_repo);

    return help_message;

}
