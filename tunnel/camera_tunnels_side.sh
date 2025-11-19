#!/bin/sh

DAFNE_MASTER="dafne@100.99.169.60"

echo "Setting up SSH tunnels for side camera streams (even ports)..."

ssh -f -N \
    -L 8080:10.10.60.96:8080 -L 8082:10.10.60.97:8080 \
    -L 8084:10.10.60.98:8080 -L 8086:10.10.60.99:8080 \
    $DAFNE_MASTER

if [ $? -eq 0 ]; then
    echo "SSH tunnels established successfully!"
    echo ""
    echo "Side camera streams available at:"
    echo "Pi .96 - Camera 1: http://localhost:8080/stream"
    echo "Pi .97 - Camera 1: http://localhost:8082/stream" 
    echo "Pi .98 - Camera 1: http://localhost:8084/stream"
    echo "Pi .99 - Camera 1: http://localhost:8086/stream"
    echo ""
    echo "View side streams in browser:"
    echo "file://$(dirname "$(readlink -f "$0")")/camera_viewer_side.html"
    echo ""
    echo "To close tunnels: pkill -f 'ssh.*$DAFNE_MASTER'"
else
    echo "Failed to establish SSH tunnels"
    exit 1
fi
