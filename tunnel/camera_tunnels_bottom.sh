#!/bin/bash

DAFNE_MASTER="dafne@100.99.169.60"

echo "Setting up SSH tunnels for bottom camera streams (odd ports)..."

ssh -f -N \
    -L 8081:10.10.60.96:8081 -L 8083:10.10.60.97:8081 \
    -L 8085:10.10.60.98:8081 -L 8087:10.10.60.99:8081 \
    $DAFNE_MASTER

if [ $? -eq 0 ]; then
    echo "SSH tunnels established successfully!"
    echo ""
    echo "Bottom camera streams available at:"
    echo "Pi .96 - Camera 2: http://localhost:8081/stream"
    echo "Pi .97 - Camera 2: http://localhost:8083/stream"
    echo "Pi .98 - Camera 2: http://localhost:8085/stream"
    echo "Pi .99 - Camera 2: http://localhost:8087/stream"
    echo ""
    echo "View bottom streams in browser:"
    echo "file://$(dirname "$(readlink -f "$0")")/camera_viewer_bottom.html"
    echo ""
    echo "To close tunnels: pkill -f 'ssh.*$DAFNE_MASTER'"
else
    echo "Failed to establish SSH tunnels"
    exit 1
fi