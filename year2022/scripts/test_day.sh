#!/bin/bash

set -euo pipefail

DAY="$1"
PART="$2"

ACTUAL=$(cargo run --bin "day$DAY-part$PART" < "data/day$DAY/sample.txt" | tail -n 1)
echo "Actual:   $ACTUAL"

EXPECTED=$(cat "data/day$DAY/sample-output-part$PART.txt")
echo "Expected: $EXPECTED"

if [ $ACTUAL == $EXPECTED ]; then
    printf "\u2714 Correct\n"
else
    printf "\u274c Failed"
    exit 1
fi