#!/bin/bash

# The closest to point <0,0,0> is the particle with the smallest Manhattan acceleration.
# Just in case, sorting is also done on velocity and position

awk -v FPAT="[0-9]+" '{printf "%d %d %d %d\n", $1 + $2 + $3, $4 + $5 + $6, $7 + $8 + $9, NR-1}' |
    sort -n -k3,3 -k2,2 -k1,1 |
    awk '{print $NF; exit}'
