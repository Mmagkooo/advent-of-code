#!/bin/bash

set -eo pipefail

if [ ! -z "$1" ]; then
    DAY="$1"
else
    # defaults to today
    DAY=$(date +%d) # zero prefixed
fi

set -u

mkdir -p "src/bin/day$DAY-"{part1,part2}
code -r "src/bin/day$DAY-"{part1,part2}"/main.rs"

mkdir -p "data/day$DAY"
./scripts/fetch_input.sh $DAY
code -r "data/day$DAY/"{sample.txt,input.txt}
