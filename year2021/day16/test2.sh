#!/bin/bash

function check() {
    input="$1"
    expected="$2"
    [ $(echo "$input" | python sol2.py | tail -n 1) == "$expected" ] || echo "err $input"
}

check C200B40A82 3
check 04005AC33890 54
check 880086C3E88112 7
check CE00C43D881120 9
check D8005AC2A8F0 1
check F600BC2D8F 0
check 9C005AC2F8F0 0
check 9C0141080250320F1802104A08 1
