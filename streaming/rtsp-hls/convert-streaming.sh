#!/bin/bash

PI_IP=192.168.1.44
PI_PORT=8554

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mkdir -p "$SCRIPT_DIR/hls"

# Add timestamp to logs
# exec > >(awk '{ print strftime("[%Y-%m-%d %H:%M:%S]"), "[** CONVERT STREAMING **]", $0; fflush(); }') 2>&1

STREAM_URL="rtsp://$PI_IP:$PI_PORT/cam"

echo "[convert-streaming.sh] Starting ffmpeg with RSTP input: $STREAM_URL"

ffmpeg \
  -fflags nobuffer \
  -flags low_delay \
  -thread_queue_size 2048 \
  -probesize 5M \
  -analyzeduration 10000000 \
  -f rtsp \
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

