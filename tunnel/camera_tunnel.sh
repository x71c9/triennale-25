#!/bin/sh

DAFNE_MASTER="dafne@100.99.169.60"

if [ $# -eq 0 ]; then
    echo "Usage: $0 <camera_number>"
    echo "Camera numbers: 1-8"
    echo "  Cameras 1,3,5,7: Side cameras (even ports)"
    echo "  Cameras 2,4,6,8: Bottom cameras (odd ports)"
    exit 1
fi

CAMERA_NUM=$1

if ! echo "$CAMERA_NUM" | grep -qE '^[1-8]$'; then
    echo "Error: Camera number must be between 1 and 8"
    exit 1
fi

LOCAL_PORT="8080"

_get_camera_info() {
    case $CAMERA_NUM in
        1) PI_IP="10.10.60.96"; REMOTE_PORT="8080"; TYPE="side" ;;
        2) PI_IP="10.10.60.96"; REMOTE_PORT="8081"; TYPE="bottom" ;;
        3) PI_IP="10.10.60.97"; REMOTE_PORT="8080"; TYPE="side" ;;
        4) PI_IP="10.10.60.97"; REMOTE_PORT="8081"; TYPE="bottom" ;;
        5) PI_IP="10.10.60.98"; REMOTE_PORT="8080"; TYPE="side" ;;
        6) PI_IP="10.10.60.98"; REMOTE_PORT="8081"; TYPE="bottom" ;;
        7) PI_IP="10.10.60.99"; REMOTE_PORT="8080"; TYPE="side" ;;
        8) PI_IP="10.10.60.99"; REMOTE_PORT="8081"; TYPE="bottom" ;;
    esac
}

_get_camera_info

echo "Setting up SSH tunnel for camera $CAMERA_NUM ($TYPE camera)..."

ssh -f -N -L $LOCAL_PORT:$PI_IP:$REMOTE_PORT $DAFNE_MASTER

if [ $? -eq 0 ]; then
    echo "SSH tunnel established successfully!"
    echo ""
    echo "Camera $CAMERA_NUM stream available at: http://localhost:$LOCAL_PORT/stream"
    echo ""
    echo "View stream in browser:"
    echo "file://$(dirname "$(readlink -f "$0")")/camera_viewer.html"
    echo ""
    echo "View with mpv:"
    echo "mpv http://localhost:$LOCAL_PORT/stream"
    echo ""
    echo "To close tunnel: pkill -f 'ssh.*-L $LOCAL_PORT:'"
else
    echo "Failed to establish SSH tunnel"
    exit 1
fi