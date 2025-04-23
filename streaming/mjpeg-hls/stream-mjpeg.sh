#!/bin/bash

# Command to run on the RPi to start streaming MJPEG

/usr/bin/camera-streamer \
    --camera-path=/dev/video0 \
    --camera-format=JPEG \
    --camera-width=1920 \
    --camera-height=1080 \
    --camera-fps=30 \
    --camera-nbufs=3 \
    --http-listen=0.0.0.0 \
    --http-port=8080 \
    --camera-video.disabled
    --log-verbose

