#!/bin/bash
cd /home/dafne/triennale-25 || exit 1
exec nix-shell shell.nix --run "cd composer && cargo run start"

