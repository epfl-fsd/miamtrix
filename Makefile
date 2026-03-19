SHELL := /bin/bash

.PHONY: up version-patch version-minor version-major down new-major-version new-minor-version new-patch-version commit tag release
up:
	export $(cat /keybase/team/epfl_matrix/.env | xargs); docker compose up -d

down:
	docker compose down -v

## Bounce patch version
version-patch: bump-version.sh
	./bump-version.sh -p

## Bounce minor version
version-minor: bump-version.sh
	./bump-version.sh -m

## Bounce major version
version-major: bump-version.sh
	./bump-version.sh -M

new-major-version: version-major commit tag release
new-minor-version: version-minor commit tag release
new-patch-version: version-patch commit tag release

commit:
	git add Cargo.*
	NEW_VERSION=$$(grep -m 1 '^version =' Cargo.toml | sed 's/version = //;s/"//g') && \
	git commit -m "feat: bump version to $$NEW_VERSION" && \
	git push

tag:
	NEW_VERSION=$$(grep -m 1 '^version =' Cargo.toml | sed 's/version = //;s/"//g') && \
	git tag v$$NEW_VERSION
	git push origin v$$NEW_VERSION

release: create-gh-release.sh
	create-gh-release.sh
