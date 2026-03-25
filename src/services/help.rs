use crate::config::CONFIG;

pub fn get_help() -> String {
    let config = CONFIG.get().expect("Failed to load config");

    let help_message = format!("\
    Available commands: `!menu`, `!yum`, `!oslf`, `!help`
    `!schedule` [day] [<command>]       Schedules a command to run automatically at 11:30 AM on the specified day(s). Example: `!schedule -c mon !menu hopper` runs the !menu hopper command every Monday at 11:30 AM.
    `!menu` [restaurant] [filter]       Get a restaurant's menu with an optional filter (e.g., pizza)
    `!yum`  [keyword] [-a <alergen>]    Search all EPFL menus for a food (e.g., pizza), Use `!` to exclude (e.g., !fish), Use `-a` to specify an alergen (e.g., -a gluten).
    `!oslf`                             List locations of french fries across EPFL's campus.
    `!list`                             List all restaurants across EPFL's campus.
    `!help`                             Get help with commands.
    version: `{}`, [source code]({}).", config.bot_version, config.bot_repo);

    return help_message;

}
