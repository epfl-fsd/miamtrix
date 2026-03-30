# Miamtrix
<div align="center">
<p>
  <img src="logo.jpg" alt="Miamtrix Logo" width="150" height="150" style="border-radius: 50%;">
</p>
<p>
  <a href="http://www.freepik.com">Designed by catalyststuff / Freepik</a>
</p>
</div>

Miamtrix is a Matrix bot that helps you check the daily menus at EPFL.

## Commands

- `!schedule <SubCommand> [OPTIONS]`
> Schedule a command with `create` Sub command or list all task of your room with `-l, --list` Sub command.

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
