#!/bin/bash

set -euo pipefail

DAY="$1"

mkdir -p "src/bin/day$DAY-"{part1,part2}
code -r "src/bin/day$DAY-"{part1,part2}"/main.rs"

mkdir "input/day$DAY"
code -r "input/day$DAY/"{sample.txt,input.txt}
