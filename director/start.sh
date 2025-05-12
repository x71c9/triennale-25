#!/bin/bash

HOME=/home/dafne

# Trap SIGINT (Ctrl+C) to kill background jobs
trap 'echo -e "\nStopping..."; jobs -p | xargs -r kill; exit' INT

cargo run robots 2 init --no-dry-run
cargo run robots 3 init --no-dry-run
cargo run robots 4 init --no-dry-run

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

  rand_val=$((RANDOM % 10))
  echo "-- Sprinkler random value: $rand_val. Activating if -eq 1"
  if [ "$rand_val" -eq 1 ]; then
    echo "Sleeping for 4 minutes before coordinated robot move..."
    sleep 240

    for rid in 2 3 4; do
      move_cmd="cargo run robots $rid move 0 4.5 --no-dry-run"
      echo -e "\n[$(date +'%H:%M:%S')] Moving robot $rid to position 4.5"
      $move_cmd
      echo "Sleeping 30 seconds before next robot..."
      sleep 30
    done

    echo "Sleeping for 2 minutes before coordinated robot move..."
    sleep 120

    sprinkler_id=$(( (RANDOM % 3) + 1 ))
    scmd="cargo run s $sprinkler_id on --no-dry-run"
    echo -e "\n[$(date +'%H:%M:%S')] Running: $scmd"
    $scmd
    sleep 20
    scmd="cargo run s $sprinkler_id off --no-dry-run"
    echo -e "\n[$(date +'%H:%M:%S')] Running: $scmd"
    $scmd
  fi

  sleep_time=$(( (RANDOM % 30) + 30 ))
  echo "Sleeping for $sleep_time seconds..."
  for ((i=sleep_time; i>0; i--)); do
    printf "Next command in %2d seconds...\r" "$i"
    sleep 1
  done
  echo -e "Launching next command...        "
done

