# Miamtrix
<div align="center">
<p>
  <img src="logo.jpg" alt="Miamtrix Logo" width="150" height="150" style="border-radius: 50%;">
</p>
<p>
  <a href="http://www.freepik.com">Designed by catalyststuff / Freepik</a>
</p>
</div>

**Miamtrix** is a fully-featured, asynchronous **Matrix Bot** written in **Rust** designed to keep EPFL students and staff informed about their daily cafeteria menus directly within their Matrix rooms.

### What does it do?
Miamtrix fetches and parses the latest daily menus from the EPFL campus restaurants API. Instead of manually checking the web, users can simply send commands to Miamtrix in any Matrix room to instantly get formatted, up-to-date food options.

### Key Features
- **Live Menu Fetching**: Instantly query daily menus from all EPFL campus cafeterias.
- **Advanced Filtering**: Search for specific meals using keywords (e.g., `veg`, `asian`, `pizza`), filter by restaurant, or exclude specific allergens.
- **Automated Scheduling**: Never miss out on your favorite meals! You can schedule the bot to automatically fetch and post the menu at specific times (e.g., every weekday at 11:30) using cron expressions. Schedules are persistently secured in a **PostgreSQL** database.
- **High Performance**: Built atop `tokio`, Miamtrix handles concurrent and asynchronous events with minimal resource footprint. It leverages the official `matrix-sdk` for end-to-end communication and `diesel-async` for database operations.

Whether you're looking for where to find fries today (`!oslf`) or want to subscribe your team's room to a daily menu broadcast, Miamtrix has you covered!

## Commands

- `!schedule <SubCommand> [OPTIONS]`
> Schedule a command with `create` Sub command or list all task of your room with `-l, --list` Sub command.
> Create Sub command has some options :
> `-j | --job` for specify the command to execute
> `-h | --hour` for specify the hour when the command will be executed (by default 11:30)
> `-d | --day` for specify the day when the command will be executed (by default mon-fri)
> Delete a cron with `delete` Sub command, you need to specify the name of the task with `-n, --name` flag.

- `!oslf`
> Returns all menus that contain fries.

- `!menu [restaurant] [filter]`
> Returns the daily menu for the restaurant specified in the parameter. If no restaurant is specified, all daily menus are returned.

- `!yum [keyword] -a` (e.g., veg, asian, dessert)
> Returns all restaurants and their menus that match with the specified keyword. You can also exclude keyword with `!` (e.g., !fish). If you have some alergen exclude yours with the flag -a (e.g., -a gluten).

- `!list`
> List all restaurant across EPFL's campus.

- `!help`
> Lists all available commands and their explanations, along with the bot's version and source code link.

## Makefile Commands

This project uses a Makefile to simplify development and versioning. You can run the following commands in your terminal:

- `make up`
> Starts the bot in the background using Docker Compose (`docker compose up -d`).

- `make down`
> Stops the Docker containers and removes the associated volumes (`docker compose down -v`).

- `make new-major-version`
> Change major version with commit, tag and gh release.

- `new-minor-version`
> Change minor version with commit, tag and gh release.

- `make new-patch-version`
> Change patch version with commit, tag and gh release.

- `make release`
> Create a gh release with a changelog and the latest tag.

- `make tag`
> Create a tag with the actual version and push.

- `make commit`
> Commit Cargo's file change with a message bump version.

- `make version-major`
> Executes the versioning script (`./bump-version.sh`) to bump major version.

- `make version-minor`
> Executes the versioning script (`./bump-version.sh`) to bump minor version.

- `make version-patch`
> Executes the versioning script (`./bump-version.sh`) to bump patch version.

## Configuration As Code
The configuration as code is made for openshift with Ansible playbook : [miamtrix's playbook](https://github.com/epfl-si/sopec/tree/feature/miamtrix)

## Quick Starting

Before choosing your environmnent. Don't forget to create a matrix account for your bot.

### Development (Dev)

To run the bot locally for development:

1. Copy the `.env.example` to `.env` and fill in your credentials:
   ```bash
   cp .env.example .env
   ```
2. Start the local PostgreSQL database using Docker Compose:
   ```bash
   docker compose -f docker-compose.dev.yml up -d
   ```
3. Run the application using Cargo:
   ```bash
   cargo run
   ```

### Production (Prod)

To deploy the bot in a production environment:

1. Copy the `.env.example` to `.env` and fill in your credentials, especially the `DATABASE_URL` pointing to your production database:
   ```bash
   cp .env.example .env
   ```
2. Start the bot as a background service using the provided Makefile command:
   ```bash
   make up
   ```
   *Note: Ensure your production environment has access to the configured PostgreSQL database before starting the container.*
