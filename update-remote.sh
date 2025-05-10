#!/bin/bash

# Remote configuration
REMOTE_USER="dafne"
REMOTE_HOST="${1:-dafne-master}"  # default to hostname, but allow override via argument
REMOTE_PATH="/home/dafne/triennale-25"

echo "Connecting to $REMOTE_USER@$REMOTE_HOST..."
ssh "${REMOTE_USER}@${REMOTE_HOST}" "cd ${REMOTE_PATH} && git pull"

