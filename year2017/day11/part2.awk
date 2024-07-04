#!/usr/bin/awk -f

function min(a, b) {
    return a < b ? a : b;
}

function clone_arr(arr_source, arr_target, i) {
    if (length(arr_target) != 0) {
        print "arr_target not empty";
        exit 1;
    }

    for (i in arr_source) {
        arr_target[i] = arr_source[i];
    }
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
    max_dist = -1;
}

{
    dirs[$0]++;
    even_out(dirs, "s", "n");
    even_out(dirs, "sw", "ne");
    even_out(dirs, "nw", "se");

    combine(dirs, "n", "se", "ne");
    combine(dirs, "ne", "s", "se");
    combine(dirs, "se", "sw", "s");
    combine(dirs, "s", "nw", "sw");
    combine(dirs, "n", "sw", "nw");
    combine(dirs, "nw", "ne", "n");

    curr_dist = 0;
    for (dir in dirs) {
        curr_dist += dirs[dir];
    }

    if (curr_dist > max_dist) {
        max_dist = curr_dist;
    }
}

END{
    print max_dist;
}
