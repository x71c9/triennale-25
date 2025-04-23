#!/bin/bash

PI_IP=192.168.1.40
PI_PORT=8080

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mkdir -p "$SCRIPT_DIR/hls"

# Add timestamp to logs
# exec > >(awk '{ print strftime("[%Y-%m-%d %H:%M:%S]"), "[** CONVERT STREAMING **]", $0; fflush(); }') 2>&1

STREAM_URL="http://$PI_IP:$PI_PORT/stream"

echo "[convert-streaming.sh] Starting ffmpeg with MJPEG input: $STREAM_URL"

ffmpeg \
  -fflags nobuffer \
  -flags low_delay \
  -thread_queue_size 2048 \
  -probesize 5M \
  -analyzeduration 10000000 \
  -f mjpeg \
  -re \
  -i "$STREAM_URL" \
  -vf "fps=15,scale=1280:720,format=yuv420p" \
  -c:v libx264 \
  -profile:v baseline \
  -preset ultrafast \
  -g 30 \
  -sc_threshold 0 \
  -f hls \
  -hls_time 2 \
  -hls_list_size 5 \
  -hls_flags delete_segments+omit_endlist+split_by_time \
  -hls_init_time 0.1 \
  "$SCRIPT_DIR/hls/index.m3u8"

