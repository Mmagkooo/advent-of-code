#!/usr/bin/awk -f

# Only the (path, last_pin, path_value) are meant to be provided when calling this function.
# The rest are local variables that are specified to prevent shadowing in recursive calls.
function find_max_path(path, last_pin, path_value, parts, new_path, rest_path_value, new_total_path_value, max_path_value) {
    max_path_value = path_value;

    for (edge in edges[last_pin]) {
        # consider edge if not already in path
        if (!index(path, edge)) {
            split(edge, parts, EDGE_SEP);
            new_last_pin = (last_pin == parts[1]) ? parts[2] : parts[1];
            new_path = path wrap_edge(edge);

            rest_path_value = find_max_path(new_path, new_last_pin, parts[1] + parts[2]);
            new_total_path_value = path_value + rest_path_value;
            if (new_total_path_value > max_path_value) {
                max_path_value = new_total_path_value;
            }
        }
    }

    return max_path_value;
}

function wrap_edge(edge) {
    return EDGE_CONNECTOR edge EDGE_CONNECTOR;
}

BEGIN{
    EDGE_SEP = "/";
    EDGE_CONNECTOR = "-";
}

{
    split($0, parts, EDGE_SEP);
    edges[parts[1]][$0] = 1;
    edges[parts[2]][$0] = 1;
}

END{
    print find_max_path("", 0, 0);
}