SHELL := /bin/bash

.PHONY: up
up:
	docker compose up -d

.PHONY: version

version: bump-version.sh
	./bump-version.sh

.PHONY: d
d:
	docker compose down -v
