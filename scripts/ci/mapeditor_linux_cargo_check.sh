#!/usr/bin/env bash
# Linux CI helper: simthing-mapeditor pulls Bevy/alsa; install deps when apt is available.
set -euo pipefail

if [[ "$(uname -s)" == "Linux" ]] && command -v apt-get >/dev/null 2>&1; then
  export DEBIAN_FRONTEND=noninteractive
  sudo apt-get update -qq
  sudo apt-get install -y -qq libasound2-dev libudev-dev libxkbcommon-dev
fi

cargo check -p simthing-mapeditor
