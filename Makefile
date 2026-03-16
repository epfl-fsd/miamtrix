SHELL := /bin/bash

.PHONY: up
up:
	export $(cat /keybase/team/epfl_matrix/.env | xargs); docker compose up -d

.PHONY: version-patch
## Bounce patch version
version-patch: bump-version.sh
	./bump-version.sh -p

.PHONY: version-minor
## Bounce minor version
version-minor: bump-version.sh
	./bump-version.sh -m

.PHONY: version-major
## Bounce major version
version-major: bump-version.sh
	./bump-version.sh -M

.PHONY: down
down:
	docker compose down -v
