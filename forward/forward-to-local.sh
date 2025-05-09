#!/bin/bash

#!/bin/bash

# SSH port forwards to run in background
ports=(6001 6002 6003 6004 6005 6006 6007 6008)

for port in "${ports[@]}"; do
    ssh -f -N -L "$port":localhost:"$port" dafne@dafne-master
done

echo "All port forwardings started."

