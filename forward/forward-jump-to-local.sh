#!/bin/bash

ssh -f -N -L 8080:dafne-pi-0.local:8080 -J dafne@dafne-master pi@dafne-pi-0.local
