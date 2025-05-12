#!/bin/bash

ffmpeg \
-i http://localhost:6001/stream \
-i http://localhost:6002/stream \
-i http://localhost:6003/stream \
-i http://localhost:6004/stream \
-i http://localhost:6005/stream \
-i http://localhost:6006/stream \
-i http://localhost:6007/stream \
-i http://localhost:6008/stream \
-filter_complex "\
[0:v]scale=320:240[t0]; \
[1:v]scale=320:240[t1]; \
[2:v]scale=320:240[t2]; \
[3:v]scale=320:240[t3]; \
[4:v]scale=320:240[t4]; \
[5:v]scale=320:240[t5]; \
[6:v]scale=320:240[t6]; \
[7:v]scale=320:240[t7]; \
[t0][t1][t2][t3]hstack=4[top]; \
[t4][t5][t6][t7]hstack=4[bottom]; \
[top][bottom]vstack[out]" \
-map "[out]" -f nut - | ffplay -
