#!/bin/bash

HOME=/home/dafne
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PYTHON_SCRIPT="$SCRIPT_DIR/../new-robots/move_motor.py"

SPARKLING_TIME=20
WAIT_TIME=$((600 - SPARKLING_TIME))
ROBOT_MOVE_INTERVAL=120  # 2 minutes

# Trap SIGINT (Ctrl+C) to kill background jobs
trap 'echo -e "\nStopping..."; jobs -p | xargs -r kill; exit' INT

# Function to handle sparkling (relay control) - runs in background
_sparkling_loop() {
  while true; do
    echo "[$(date)] Turning ON s0, s1 and s2..."
    curl "http://192.168.125.3/s0?state=on"
    curl "http://192.168.125.3/s1?state=on"
    curl "http://192.168.125.3/s2?state=on"

    echo "[$(date)] Waiting $SPARKLING_TIME seconds with relays ON..."
    sleep $SPARKLING_TIME

    echo "[$(date)] Turning OFF s0, s1 and s2..."
    curl "http://192.168.125.3/s0?state=off"
    curl "http://192.168.125.3/s1?state=off"
    curl "http://192.168.125.3/s2?state=off"

    echo "[$(date)] Waiting $WAIT_TIME seconds before next cycle..."
    sleep $WAIT_TIME
  done
}

# Function to handle robot movements - runs in background
_robot_loop() {
  while true; do
    echo "[$(date)] Starting coordinated robot movement sequence..."
    
    for motor_id in 1 2 3 4; do
      # Generate random position between 2500-5000mm
      position=$(awk 'BEGIN{srand(); printf "%.1f", 2500 + rand() * 2500}')
      
      echo "[$(date)] Moving robot motor $motor_id to position $position mm..."
      python3 "$PYTHON_SCRIPT" --motor-id $motor_id --position $position >/dev/null 2>&1 &
      
      # Random delay between 60-120 seconds (1-2 minutes) between robots
      if [ $motor_id -lt 4 ]; then
        delay=$((60 + RANDOM % 61))
        echo "[$(date)] Waiting $delay seconds before next robot..."
        sleep $delay
      fi
    done
    
    echo "[$(date)] All robots movement commands sent. Waiting $ROBOT_MOVE_INTERVAL seconds before next sequence..."
    sleep $ROBOT_MOVE_INTERVAL
  done
}

echo "[$(date)] Starting sparkling loop..."
_sparkling_loop &

echo "[$(date)] Starting robot loop..."
_robot_loop &

echo "[$(date)] Both loops started. Waiting..."
wait

