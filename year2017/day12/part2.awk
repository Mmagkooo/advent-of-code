#!/usr/bin/awk -f

BEGIN{FS=" <-> "}

{
    from = $1;
    split($2, to_list, ", ");
    for (to_i in to_list) {
        to = to_list[to_i];
        pipe[from][to] = 1;
    }
}

function count_members(id, seen) {
    if (id in seen) {
        return;
    }
    seen[id] = 1;

    for (neighbor in pipe[id]) {
        count_members(neighbor, seen);
    }
}

END{
    for (id in pipe) {
        if (id in seen) {
            # id already belongs to a group
            continue;
        }

        ++groups;
        count_members(id, seen);
    }
    print groups;
}
