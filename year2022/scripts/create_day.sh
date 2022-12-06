#!/bin/bash

set -euo pipefail

DAY="day$1"

mkdir "$DAY-"{part1,part2}
mkdir "input/$DAY"
touch "input/$DAY/"{sample.txt,input.txt}
