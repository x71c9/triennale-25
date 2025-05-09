#!/bin/bash

ssh -f -N -L 6001:localhost:8080 pi@dafne-pi-0.local
ssh -f -N -L 6002:localhost:8081 pi@dafne-pi-0.local

ssh -f -N -L 6003:localhost:8080 pi@dafne-pi-1.local
ssh -f -N -L 6004:localhost:8081 pi@dafne-pi-1.local

ssh -f -N -L 6005:localhost:8080 pi@dafne-pi-2.local
ssh -f -N -L 6006:localhost:8081 pi@dafne-pi-2.local

ssh -f -N -L 6007:localhost:8080 pi@dafne-pi-3.local
ssh -f -N -L 6008:localhost:8081 pi@dafne-pi-3.local

