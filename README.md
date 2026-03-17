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

- `/oslf`
> Returns all menus that contain fries.

- `/menu [restaurant] [filter]`
> Returns the daily menu for the restaurant specified in the parameter. If no restaurant is specified, all daily menus are returned.

- `/miam [food type]` (e.g., veg, asian, dessert)
> Returns all restaurants and their menus that match the specified food type.

> [!NOTE]
> The `/miam` command can be merged with the `/menu` command by adding automatic parameter detection (to determine if the input is a restaurant or a food type).

## Makefile Commands

This project uses a Makefile to simplify development and versioning. You can run the following commands in your terminal:

- `make up`
> Starts the bot in the background using Docker Compose (`docker compose up -d`).

- `make version-major`
> Executes the versioning script (`./bump-version.sh`) to bump major version.

- `make version-minor`
> Executes the versioning script (`./bump-version.sh`) to bump minor version.

- `make version-patch`
> Executes the versioning script (`./bump-version.sh`) to bump patch version.

- `make down`
> Stops the Docker containers and removes the associated volumes (`docker compose down -v`).
