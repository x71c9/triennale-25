#!/bin/bash
cd /home/dafne/triennale-25 || exit 1
exec /nix/var/nix/profiles/default/bin/nix-shell /home/dafne/triennale-25/shell.nix --run "cd director && bash start.sh"

