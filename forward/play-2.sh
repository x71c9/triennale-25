#!/bin/bash

ffmpeg \
-i http://localhost:6001/stream \
-i http://localhost:6002/stream \
-filter_complex "\
[0:v]scale=320:240[t0]; \
[1:v]scale=320:240[t1]; \
[t0][t1]hstack=2[out]" \
-map "[out]" -f nut - | ffplay -

