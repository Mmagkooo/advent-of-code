#!/usr/bin/awk -f

function to_str(dict) {
    s = ""
    for (char_i in dict) {
        s = s "," dict[char_i]
    }
    return s;
}

# precedence to smaller index
function find_max_i(dict) {
    max_i = 0;
    for (candidate_i in dict) {
        if (dict[candidate_i] > dict[max_i]) {
            max_i = candidate_i;
        }
    }
    return max_i;
}

function redistribute(dict) {
    max_i = find_max_i(dict);
    distributable = dict[max_i];
    dict[max_i] = 0;

    dict_size = length(dict);
    start_cell_i = (max_i + 1) % dict_size;
    for (cell_i = start_cell_i; distributable > 0; cell_i = (cell_i + 1) % dict_size) {
        dict[cell_i] += 1;
        --distributable;
    }
}

{
    split($0, initial_blocks, " ");
    # make it 0-indexed
    for (j in initial_blocks) {
        blocks[j - 1] = initial_blocks[j];
    }
}

END{
    while (1) {
        current_stringified = to_str(blocks);
        if (current_stringified in seen) {
            print steps - seen[current_stringified];
            break;
        } else {
            seen[current_stringified] = steps;
        }
        redistribute(blocks);
        ++steps;
    }
}
