#!/usr/bin/env bash

set -e

if [ -z "$GITHUB_TOKEN" ]; then
  echo "Error GITHUB_TOKEN not found."
  exit 1
fi

PROJECT_NAME=$(grep -m 1 '^name =' Cargo.toml | sed 's/name = //;s/"//g' | tr -d ' ')
VERSION=$(grep -m 1 '^version =' Cargo.toml | sed 's/version = //;s/"//g' | tr -d ' ')

REPO_REMOTE=$(git config --get remote.origin.url)
REPO_PATH=$(echo "$REPO_REMOTE" | sed -E 's/.*github\.com[:/]([^\/]+)\/([^\.]+)(\.git)?/\1\/\2/')
REPO_ORG_OR_USR=$(echo "$REPO_PATH" | cut -d/ -f1)
REPO_NAME=$(echo "$REPO_PATH" | cut -d/ -f2)

GH_API="https://api.github.com"
GH_API_RELEASE_URL="$GH_API/repos/$REPO_ORG_OR_USR/$REPO_NAME/releases"
AUTH="Authorization: token $GITHUB_TOKEN"

function printInfo() {
  echo "--------------------------------------------------------------------------------"
  echo "Projet        : $PROJECT_NAME"
  echo "Version       : v$VERSION"
  echo "Dépôt distant : $REPO_ORG_OR_USR/$REPO_NAME"
  echo "API URL       : $GH_API_RELEASE_URL"
  echo "--------------------------------------------------------------------------------"
}

function generate_changelog() {
  LATEST_RELEASE_TAG=$(curl -s -H "$AUTH" "$GH_API/repos/$REPO_ORG_OR_USR/$REPO_NAME/releases/latest" | jq -r .tag_name)

  if [ "$LATEST_RELEASE_TAG" == "null" ] || [ -z "$LATEST_RELEASE_TAG" ]; then
    LOG_RANGE="HEAD"
  else
    LOG_RANGE="${LATEST_RELEASE_TAG}..HEAD"
  fi

  FEATURE_NOTES=$(git log $LOG_RANGE --no-merges --pretty=format:"- %s" | grep -i '\[feature\]' | sed -E 's/-\s*\[[a-zA-Z]+\]\s*/- /' || true)
  FIX_NOTES=$(git log $LOG_RANGE --no-merges --pretty=format:"- %s" | grep -i '\[fix\]' | sed -E 's/-\s*\[[a-zA-Z]+\]\s*/- /' || true)
  DOC_NOTES=$(git log $LOG_RANGE --no-merges --pretty=format:"- %s" | grep -i '\[documentation\]' | sed -E 's/-\s*\[[a-zA-Z]+\]\s*/- /' || true)
  RELEASE_NOTES=""

  if [ -n "$FEATURE_NOTES" ]; then
      RELEASE_NOTES+="### ✨ New features"$'\n'"$FEATURE_NOTES"$'\n\n'
  fi

  if [ -n "$FIX_NOTES" ]; then
    RELEASE_NOTES+="### 🐛 Fixes"$'\n'"$FIX_NOTES"$'\n\n'
  fi

  if [ -n "$DOC_NOTES" ]; then
    RELEASE_NOTES+="### 📖 Docs"$'\n'"$DOC_NOTES"$'«\n\n'
  fi

  if [ -z "$RELEASE_NOTES" ]; then
    RELEASE_NOTES="- No new features or fixes for this version."
  fi

  echo -e "\n--- Changelog Preview ---"
  echo "$RELEASE_NOTES"
  echo -e "-------------------------\n"
}

function create_release () {

  RELEASE_JSON=$(jq -n \
    --arg tag "v$VERSION" \
    --arg name "v$VERSION" \
    --arg body "$RELEASE_NOTES" \
    '{tag_name: $tag, name: $name, body: $body, draft: false, prerelease: false}')

  RELEASE_RESPONSE=$(curl -s -X POST -H "$AUTH" -H "Accept: application/vnd.github.v3+json" -d "$RELEASE_JSON" "$GH_API_RELEASE_URL")

  RELEASE_ID=$(echo "$RELEASE_RESPONSE" | jq -r .id)
  RELEASE_ERRORS=$(echo "$RELEASE_RESPONSE" | jq -r .message)

  if [ "$RELEASE_ID" == "null" ] || [ -z "$RELEASE_ID" ]; then
    echo "Failed to create release error: $RELEASE_ERRORS"
    exit 1
  else
    echo "Create release successfuly $RELEASE_ID"
    RELEASE_URL=$(echo "$RELEASE_RESPONSE" | jq -r .html_url)
    echo "Link : $RELEASE_URL"
  fi
}

printInfo
generate_changelog
create_release
