#!/usr/bin/awk -f

BEGIN{
    size=256;
    for (i = 0; i < size; ++i) list[i] = i;
    position=0;

    # there is no native ord in awk, this is a minimal subset of it needed for this task
    ord[","] = 44;
    for (i = 0; i <= 9; ++i) {
        ord[i] = 48 + i;
    }
}

{
    split($0, raw_lengths, "");
    for (i = 1; i <= length(raw_lengths); ++i) {
        lengths[i] = ord[raw_lengths[i]];
    }

    split("17, 31, 73, 47, 23", extra_lengths, ", ");
    for (i = 1; i <= length(extra_lengths); ++i) {
        lengths[length(lengths) + 1] = extra_lengths[i];
    }
}

END{
    rounds = 64;
    while (rounds--) {
        for (length_i = 1; length_i <= length(lengths); ++length_i) {
            current_length = lengths[length_i];
            end = (position + current_length - 1 + size) % size;

            i = position;
            j = end;
            steps = int(current_length / 2);
            while (steps--) {
                tmp = list[i];
                list[i] = list[j];
                list[j] = tmp;

                i = (i + 1 + size) % size;
                j = (j - 1 + size) % size;
            }

            position = (end + 1 + skip_size++ + size) % size;
        }
    }

    for (i = 0; i < size;) {
        next_stop = i + 16;
        dense_hash = list[i++];
        for (; i < next_stop; ++i) {
            dense_hash = xor(dense_hash, list[i]);
        }
        printf "%02x", dense_hash;
    }
    printf "\n";
}
