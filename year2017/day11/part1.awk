#!/usr/bin/awk -f

function min(a, b) {
    return a < b ? a : b;
}

function even_out(dirs, d1, d2, min_dir) {
    min_dir = min(dirs[d1], dirs[d2]);
    dirs[d1] -= min_dir;
    dirs[d2] -= min_dir;
}

function combine(dirs, d1, d2, d_middle, min_dir) {
    min_dir = min(dirs[d1], dirs[d2]);
    dirs[d_middle] += min_dir;
    dirs[d1] -= min_dir;
    dirs[d2] -= min_dir;
}

BEGIN{
    RS=",";
}

{
    dirs[$0]++;
}

END{
    even_out(dirs, "s", "n");
    even_out(dirs, "sw", "ne");
    even_out(dirs, "nw", "se");

    combine(dirs, "n", "se", "ne");
    combine(dirs, "ne", "s", "se");
    combine(dirs, "se", "sw", "s");
    combine(dirs, "s", "nw", "sw");
    combine(dirs, "n", "sw", "nw");
    combine(dirs, "nw", "ne", "n");

    for (dir in dirs) {
        min_dist += dirs[dir];
    }
    print min_dist;
}
