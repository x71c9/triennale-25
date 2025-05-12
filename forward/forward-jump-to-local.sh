#!/bin/bash

# eval "$(ssh-agent -s)"
# ssh-add ~/.ssh/id_rsa  # or your specific key

# ssh -f -N -L 8080:dafne-pi-0.local:8080 -J dafne@dafne-master pi@dafne-pi-0.local

# ssh -J dafne@dafne-master pi@dafne-pi-0.local -L 6001:localhost:8080 -N
