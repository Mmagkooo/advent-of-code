#!/bin/bash

set -euo pipefail

awk '{for (i = 0; i < 128; ++i) print $0 "-" i}' \
| ../day10/part2.awk \
| ./hex_to_bin.awk \
| ./count_regions.awk
