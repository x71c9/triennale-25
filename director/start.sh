#!/bin/bash

# Trap SIGINT (Ctrl+C) to kill background jobs
trap 'echo -e "\nStopping..."; jobs -p | xargs -r kill; exit' INT

cargo run robots init 2 init --no-dry-run
sleep 60
cargo run robots init 3 init --no-dry-run
sleep 60
cargo run robots init 4 init --no-dry-run
sleep 60

while true; do
  # Generate random robot_id: 2, 3, or 4
  robot_id=$(( (RANDOM % 3) + 2 ))
  move_dir=$(( RANDOM % 6 ))
  move_amount=$(awk -v min=0.1 -v max=1 'BEGIN{srand(); printf "%.2f\n", min + rand() * (max - min)}')

  # Build and display the command
  cmd="cargo run robots $robot_id move $move_dir $move_amount --no-dry-run"
  echo -e "\n[$(date +'%H:%M:%S')] Running: $cmd"

  # Run the command in the background
  $cmd &

  if [ $((RANDOM % 10)) -eq 0 ]; then
    robot_id=$(( (RANDOM % 3) + 1 ))
    scmd="cargo run sparklings $sparkling_id on --no-dry-run"
    echo -e "\n[$(date +'%H:%M:%S')] Running: $scmd"
    sleep 20
    scmd="cargo run sparklings $sparkling_id off --no-dry-run"
    echo -e "\n[$(date +'%H:%M:%S')] Running: $scmd"
  fi

  # Sleep between 15 and 30 seconds with countdown
  sleep_time=$(( (RANDOM % 16) + 15 ))
  echo "Sleeping for $sleep_time seconds..."
  for ((i=sleep_time; i>0; i--)); do
    printf "Next command in %2d seconds..." "$i"
    sleep 1
  done
  echo -e "Launching next command...        "
done
