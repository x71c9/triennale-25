#!/bin/bash

HOME=/home/dafne

SPARKLING_TIME=60

# TODO Remove after Robots are fixed
while true; do
  echo "[$(date)] Turning ON s1 and s2..."
  curl "http://192.168.125.3/s1?state=on"
  curl "http://192.168.125.3/s2?state=on"

  echo "[$(date)] Waiting 60 seconds with relays ON..."
  sleep $SPARKLING_TIME

  echo "[$(date)] Turning OFF s1 and s2..."
  curl "http://192.168.125.3/s1?state=off"
  curl "http://192.168.125.3/s2?state=off"

  # Wait the remaining time to make it 10 minutes total (600 - 60 = 540 seconds)
  echo "[$(date)] Waiting 540 seconds before next cycle..."
  sleep 540
done


# Trap SIGINT (Ctrl+C) to kill background jobs
trap 'echo -e "\nStopping..."; jobs -p | xargs -r kill; exit' INT

log() {
  # echo -e "[$(date +'%H:%M:%S')] $1"
  echo -e "$1"
}

countdown() {
  local seconds=$1
  local msg=$2
  for ((i=seconds; i>0; i--)); do
    # echo "[$(date +'%H:%M:%S')] $msg in $i seconds..."
    echo "$msg in $i seconds..."
    sleep 1
  done
}

log "Initializing robots..."
cargo run robots 2 init --no-dry-run
cargo run robots 3 init --no-dry-run
cargo run robots 4 init --no-dry-run

log "Entering main loop..."
while true; do

  current_hour=$(date +%H)
  if (( current_hour < 8 || current_hour >= 21 )); then
    log "Out of working hours (08:00-21:00). Sleeping 1 hour..."
    sleep 60 * 60
    continue
  fi

  # Generate random robot_id: 2, 3, or 4
  robot_id=$(( (RANDOM % 3) + 2 ))
  # move_position=$(echo "$(( RANDOM % 5 )).5") # from .5 to 4.5
  move_position=$(awk -v min=0.5 -v max=4.5 'BEGIN{srand(); printf "%.2f\n", min + rand() * (max - min)}')
  move_amount=$(awk -v min=0.1 -v max=1 'BEGIN{srand(); printf "%.2f\n", min + rand() * (max - min)}')

  cmd="cargo run robots $robot_id move $move_position $move_amount --no-dry-run"
  log "Running: $cmd"
  $cmd &
  log "Started: $cmd"

  rand_val=$((RANDOM % 10))
  log "---------------- Sprinkler random value: $rand_val. Activating if -eq 1"

  if [ "$rand_val" -eq 1 ]; then
    log "Preparing for [SCANNING] coordinated robot move..."
    countdown 60 "[SCANNING] Coordinated robot move starts"

    for rid in 2 3 4; do
      move_cmd="cargo run robots $rid move 4.5 1.0 --no-dry-run"
      log "[SCANNING] Moving robot $rid to position 4.5"
      $move_cmd &
      log "[SCANNING] Finished moving robot $rid"
      countdown 30 "[SCANNING] Next robot move"
    done

    log "[SCANNING] Waiting before sprinkler activation..."
    countdown 120 "[SCANNING] Sprinkler activation"

    sprinkler_id=$(( (RANDOM % 3) + 1 ))
    scmd_on="cargo run s $sprinkler_id on --no-dry-run"
    log "[WATER] Running: $scmd_on"
    $scmd_on
    countdown $SPARKLING_TIME "[WATER] Sprinkler off"

    scmd_off="cargo run s $sprinkler_id off --no-dry-run"
    log "[WATER] Running: $scmd_off"
    $scmd_off
  fi

  sleep_time=$(( (RANDOM % 30) + 30 ))
  log "Sleeping for $sleep_time seconds..."
  countdown $sleep_time "Next command"
done

