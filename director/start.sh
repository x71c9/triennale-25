#!/bin/bash

HOME=/home/dafne
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PYTHON_SCRIPT="$SCRIPT_DIR/../new-robots/move_motor.py"

SPARKLING_TIME=20
WAIT_TIME=$((600 - SPARKLING_TIME))
ROBOT_MOVE_INTERVAL=120  # 2 minutes
SYNC_EVERY=5  # Sync every 5 iterations
iteration_count=0

# Trap SIGINT (Ctrl+C) and SIGTERM (systemd stop) to kill background jobs
trap 'echo -e "\nStopping..."; jobs -p | xargs -r kill; exit' INT TERM

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
    iteration_count=$((iteration_count + 1))
    
    # Check if it's time for synchronized movement (including first iteration)
    if [ $iteration_count -eq 1 ] || [ $((iteration_count % SYNC_EVERY)) -eq 0 ]; then
      echo "[$(date)] SYNC MODE: Moving all robots to synchronized positions..."
      
      # Randomly decide direction: 0 = 2500->5000, 1 = 5000->2500
      if [ $((RANDOM % 2)) -eq 0 ]; then
        first_position=2500
        second_position=5000
        echo "[$(date)] Random direction: 2500 -> 5000"
      else
        first_position=5000
        second_position=2500
        echo "[$(date)] Random direction: 5000 -> 2500"
      fi
      
      # First move all robots to first position
      for motor_id in 1 2 3 4; do
        echo "[$(date)] Moving robot motor $motor_id to SYNC position $first_position mm..."
        python3 "$PYTHON_SCRIPT" --motor-id $motor_id --position $first_position >/dev/null 2>&1 &
        sleep 1  # Small delay to avoid serial port conflicts
      done
      
      echo "[$(date)] Waiting 45 seconds before second sync move..."
      sleep 45
      
      # Then move all robots to second position
      for motor_id in 1 2 3 4; do
        echo "[$(date)] Moving robot motor $motor_id to SYNC position $second_position mm..."
        python3 "$PYTHON_SCRIPT" --motor-id $motor_id --position $second_position >/dev/null 2>&1 &
        sleep 1  # Small delay to avoid serial port conflicts
      done
      
      echo "[$(date)] Waiting 45 seconds after sync sequence..."
      sleep 45
    else
      echo "[$(date)] NORMAL MODE: Starting coordinated robot movement sequence..."
      
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
    fi
    
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

