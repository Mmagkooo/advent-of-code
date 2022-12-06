#!/bin/bash

function check() {
    num="$1"
    expected="$2"
    [ $(python sol1.py < "sample${num}.txt" | tail -n 1) == "$expected" ] || echo "err$num"
}

check 0 6
check 1 16
check 2 12
check 3 23
check 4 31
check 5 9
