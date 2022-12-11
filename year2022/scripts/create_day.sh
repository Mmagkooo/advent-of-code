#!/bin/bash

set -euo pipefail

DAY="$1"

mkdir -p "src/bin/day$DAY-"{part1,part2}
code -r "src/bin/day$DAY-"{part1,part2}"/main.rs"

mkdir "data/day$DAY"
code -r "data/day$DAY/"{sample.txt,sample-output-part1.txt,sample-output-part2.txt,input.txt}
