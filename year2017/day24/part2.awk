#!/usr/bin/awk -f

# Only the (path, last_pin, path_value) are meant to be provided when calling this function.
# The rest are local variables that are specified to prevent shadowing in recursive calls.
# The function returns a space-separated concatenation of the max_path value and length.
function find_max_path(path, last_pin, path_value, path_length, parts, new_path, rest, rest_arr, rest_path_value, rest_path_length, new_total_path_value, max_path_value, max_path_length) {
    max_path_value = path_value;
    max_path_length = path_length;

    for (edge in edges[last_pin]) {
        # consider edge if not already in path
        if (!index(path, edge)) {
            split(edge, parts, EDGE_SEP);
            new_last_pin = (last_pin == parts[1]) ? parts[2] : parts[1];
            new_path = path wrap_edge(edge);

            rest = find_max_path(new_path, new_last_pin, parts[1] + parts[2], 1);
            split(rest, rest_arr, RET_SEP);
            rest_path_value = rest_arr[1];
            rest_path_length = rest_arr[2];

            new_total_path_value = path_value + rest_path_value;
            new_total_path_length = path_length + rest_path_length;
            if (new_total_path_length > max_path_length) {
                max_path_length = new_total_path_length;
                max_path_value = new_total_path_value;
            } else if (new_total_path_length == max_path_length && new_total_path_value > max_path_value) {
                max_path_value = new_total_path_value;
            }
        }
    }

    return max_path_value RET_SEP max_path_length;
}

function wrap_edge(edge) {
    return EDGE_CONNECTOR edge EDGE_CONNECTOR;
}

BEGIN{
    EDGE_SEP = "/";
    EDGE_CONNECTOR = "-";
    RET_SEP = " ";
}

{
    split($0, parts, EDGE_SEP);
    edges[parts[1]][$0] = 1;
    edges[parts[2]][$0] = 1;
}

END{
    sol = find_max_path("", 0, 0, 0);
    split(sol, sol_parts, RET_SEP);
    print sol_parts[1];
}