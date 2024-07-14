#!/bin/bash

set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Error: $0 <DAY_NUMBER>" 1>&2
    exit 1
fi

day="$1"
directory="day$day"

if [ -d "$directory" ]; then
    echo "Error: $directory" already exists 1>&2
    exit 2
fi

mkdir "$directory"

for i in 1 2; do
    filename="$directory/part$i.awk"
    echo "#!/usr/bin/awk -f" >"$filename"
    chmod +x "$filename"
done
touch "$directory/sample.txt" "$directory/input.txt"
