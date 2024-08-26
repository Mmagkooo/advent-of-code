#!/usr/bin/awk -f

function extract_word(input, i, words) {
    patsplit(input, words, /[A-Za-z0-9]+/);
    return words[i];
}

function direction_to_index_delta(direction) {
    if (direction == "right") return 1;
    if (direction == "left") return -1;

    print "Error: invalid direction", direction;
    exit 1;
}

BEGIN{RS="\n\n"}

/^In state/ {
    split($0, rows, "\n");

    in_state = extract_word(rows[1], 3);

    next_value_map[in_state, 0] = extract_word(rows[3], 4);
    next_index_delta_map[in_state, 0] = direction_to_index_delta(extract_word(rows[4], 6));
    next_state_map[in_state, 0] = extract_word(rows[5], 4);

    next_value_map[in_state, 1] = extract_word(rows[7], 4);
    next_index_delta_map[in_state, 1] = direction_to_index_delta(extract_word(rows[8], 6));
    next_state_map[in_state, 1] = extract_word(rows[9], 4);

    next;
}

/^Begin in state/{
    split($0, rows, "\n");
    current_state = extract_word(rows[1], 4);

    total_steps = extract_word(rows[2], 6);
}

END{
    current_index = 0;

    while (total_steps--) {
        current_value = tape[current_index] || 0;
        next_state = next_state_map[current_state, current_value];
        next_value = next_value_map[current_state, current_value];
        next_index_delta = next_index_delta_map[current_state, current_value];

        tape[current_index] = next_value;
        current_state = next_state;
        current_index += next_index_delta;
    }

    diagnostic_checksum = 0;
    for (i in tape) {
        diagnostic_checksum += tape[i];
    }
    print diagnostic_checksum;
}
