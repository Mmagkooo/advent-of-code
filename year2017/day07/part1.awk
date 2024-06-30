#!/usr/bin/awk -f

BEGIN{FPAT="[a-z0-9]+"}
{ progs[$1] }
{
    for (i = 3; i <= NF; ++i) {
        parent[$i] = $1;
    }
}

END{
    for (prog in progs) {
        if (!(prog in parent)) { print prog }
    }
}
