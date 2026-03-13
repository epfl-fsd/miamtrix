#!/usr/bin/env bash

set -e

NEWVERSION=$(grep -m 1 '^version =' Cargo.toml | sed 's/version = //;s/"//g')

read -p "Do you want to change the version to $NEWVERSION ? [Yy]: " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]
then
  git add Cargo.*
  git commit -m "[version] Bump version to $NEWVERSION"
  git tag "v$NEWVERSION"
  git push
  git push --tag
else
  echo "Aborted"
fi
