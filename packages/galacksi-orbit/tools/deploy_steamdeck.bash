#!/bin/bash
# To use, create a system hosts entry that points 'steamdeck.local' to the steamdeck's IP address.
# Configure ~/ssh/.config for the correct username as well as the key that is installed on the Steam Deck's ~/.ssh/authorized_keys
# You will need to have access to the game on Steam to run builds for it (it uses Steam's API).
set -euo pipefail

# CONFIG
GAME_STEAM_TITLE="Galacksi Orbit"
BIN_NAME="galacksi-orbit"
TARGET_PATH="../../target"

RELEASE_BIN="$TARGET_PATH/release/$BIN_NAME"
STEAMDECK_HOSTNAME="steamdeck.local"
STEAM_APPS_PATH="~/.local/share/Steam/steamapps/common"
STEAM_APP_PATH="$STEAM_APPS_PATH/$GAME_STEAM_TITLE"

cargo build --release
ssh "$STEAMDECK_HOSTNAME" 'killall "$BIN_NAME" 2>/dev/null || echo -n'
scp "$RELEASE_BIN" "$STEAMDECK_HOSTNAME:$STEAM_APP_PATH"
