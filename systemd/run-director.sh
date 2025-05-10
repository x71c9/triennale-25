#!/bin/bash
cd /home/dafne/triennale-25 || exit 1
exec /nix/var/nix/profiles/default/bin/nix-shell shell.nix --run "cd director && cargo run installation start"

