#!/bin/sh

DAFNE_MASTER="dafne@100.99.169.60"
PIS=("10.10.60.96" "10.10.60.97" "10.10.60.98" "10.10.60.99")

echo "Setting up SSH tunnels for camera streams..."

ssh -f -N \
    -L 8080:10.10.60.96:8080 -L 8081:10.10.60.96:8081 \
    -L 8082:10.10.60.97:8080 -L 8083:10.10.60.97:8081 \
    -L 8084:10.10.60.98:8080 -L 8085:10.10.60.98:8081 \
    -L 8086:10.10.60.99:8080 -L 8087:10.10.60.99:8081 \
    $DAFNE_MASTER

if [ $? -eq 0 ]; then
    echo "SSH tunnels established successfully!"
    echo ""
    echo "Camera streams available at:"
    echo "Pi .96 - Camera 1: http://localhost:8080/stream"
    echo "Pi .96 - Camera 2: http://localhost:8081/stream"
    echo "Pi .97 - Camera 1: http://localhost:8082/stream" 
    echo "Pi .97 - Camera 2: http://localhost:8083/stream"
    echo "Pi .98 - Camera 1: http://localhost:8084/stream"
    echo "Pi .98 - Camera 2: http://localhost:8085/stream"
    echo "Pi .99 - Camera 1: http://localhost:8086/stream"
    echo "Pi .99 - Camera 2: http://localhost:8087/stream"
    echo ""
    echo "View all streams in browser:"
    echo "file://$(dirname "$(readlink -f "$0")")/camera_viewer_all.html"
    echo ""
    echo "To close tunnels: pkill -f 'ssh.*$DAFNE_MASTER'"
else
    echo "Failed to establish SSH tunnels"
    exit 1
fi
