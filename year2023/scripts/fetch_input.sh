#!/bin/bash

# assumes it's invoked in year directory

source ../.env

set -eu

YEAR=2023
if [ ! -z "$1" ]; then
    DAY="$1"
else
    # defaults to today
    DAY=$(date +%d) # zero prefixed
fi

STRIPPED_DAY=${DAY#"0"}
curl -L "https://adventofcode.com/$YEAR/day/$STRIPPED_DAY/input" \
    --cookie "session=$SESSION" \
    -o "data/day$DAY/input.txt"
