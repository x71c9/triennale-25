#!/bin/bash

# Remote configuration
REMOTE_USER="dafne"
REMOTE_HOST="${1:-dafne-master}"  # default to hostname, but allow override via argument
REMOTE_PATH="/home/dafne/triennale-25"
SERVICE_FILE="systemd/director.service"
SYSTEMD_DEST="/etc/systemd/system/director.service"

# Flag to optionally start the service
START_NOW=false

# Parse arguments
for arg in "$@"; do
  if [[ "$arg" == "--start-now" ]]; then
    START_NOW=true
  fi
done

echo "Connecting to $REMOTE_USER@$REMOTE_HOST..."

ssh "${REMOTE_USER}@${REMOTE_HOST}" bash << EOF
  set -e
  echo "Pulling latest changes in $REMOTE_PATH..."
  cd "$REMOTE_PATH"
  git reset --hard
  git pull --recurse-submodules
  git submodule update --init --recursive
  cd "$REMOTE_PATH/robotics"
  git checkout main

  echo "Updating systemd service definition..."
  sudo cp "$REMOTE_PATH/$SERVICE_FILE" "$SYSTEMD_DEST"
  sudo systemctl daemon-reload
  sudo systemctl reenable director.service

  if $START_NOW; then
    echo "Starting director.service..."
    sudo systemctl restart director.service
  else
    echo "Service not started now. Will take effect on next boot."
  fi
EOF

