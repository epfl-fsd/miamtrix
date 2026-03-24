SHELL := /bin/bash

PROJECT_NAME := $(shell basename $(CURDIR))
REPO_OWNER_NAME := $(shell git config --get user.name)
REPO_OWNER_EMAIL := $(shell git config --get user.email)

.PHONY: help
help:
	@echo "$$(tput bold)Available rules (alphabetical order):$$(tput sgr0)";sed -ne"/^## /{h;s/.*//;:d" -e"H;n;s/^## //;td" -e"s/:.*//;G;s/\\n## /---/;s/\\n/ /g;p;}" ${MAKEFILE_LIST}|LC_ALL='C' sort -f |awk -F --- -v n=$$(tput cols) -v i=20 -v a="$$(tput setaf 6)" -v z="$$(tput sgr0)" '{printf"%s%*s%s ",a,-i,$$1,z;m=split($$2,w," ");l=n-i;for(j=1;j<=m;j++){l-=length(w[j])+1;if(l<= 0){l=n-i-length(w[j])-1;printf"\n%*s ",-i," ";}printf"%s ",w[j];}printf"\n";}'


.PHONY: up version-patch version-minor version-major down new-major-version new-minor-version new-patch-version commit tag release
## Build image and mount docker container
up:
	export $(cat /keybase/team/epfl_matrix/.env | xargs); docker compose up -d

## Stop docker container
down:
	docker compose down -v

## Mount container postgresql for dev
db-dev:
	docker compose -f docker-compose.dev.yml up

## Bounce patch version
version-patch: bump-version.sh
	./bump-version.sh -p

## Bounce minor version
version-minor: bump-version.sh
	./bump-version.sh -m

## Bounce major version
version-major: bump-version.sh
	./bump-version.sh -M

## Change major version with commit, tag and gh release
new-major-version: version-major commit tag release

## Change minor version with commit, tag and gh release
new-minor-version: version-minor commit tag release

## Change patch version with commit, tag and gh release
new-patch-version: version-patch commit tag release

## Commit Cargo's file change with a message bump version
commit:
	git add Cargo.*
	NEW_VERSION=$$(grep -m 1 '^version =' Cargo.toml | sed 's/version = //;s/"//g') && \
	git commit -m "[version] bump version to $$NEW_VERSION" && \
	git push

## Create a tag with the actual version and push
tag:
	NEW_VERSION=$$(grep -m 1 '^version =' Cargo.toml | sed 's/version = //;s/"//g') && \
	git tag v$$NEW_VERSION
	git push --tag

## Create a gh release with a changelog and the latest tag
release: create-gh-release.sh
	./create-gh-release.sh
