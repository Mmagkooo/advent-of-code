#!/usr/bin/awk -f

BEGIN{FPAT="\\w=<[^>]*>"; DIM=3}

# Returns the `prop` of the particle at index `i` in the form of a string.
function get_stringified_prop(i, prop, j, s) {
    s = "<" particle[i][prop][0];
    for (j = 1; j < DIM; ++j) {
        s = s "," particle[i][prop][j];
    }
    return s ">";
}

{
    for (i = 1; i <= NF; ++i) {
        particle_index = NR - 1;

        split($i, prop, "=<");
        prop_name = prop[1];
        split(prop[2], values, ",");

        if (length(values) != DIM) {
            print "Invalid number of values per property";
            exit 1;
        }

        for (j = 1; j <= DIM; ++j) {
            particle[particle_index][prop_name][j - 1] = values[j];
        }
    }
}

END{
    last_changed = -1;
    for (round_counter = 0; round_counter - last_changed < 100; ++round_counter) {
        for (particle_index in particle) {
            for (j = 0; j < DIM; ++j) {
                particle[particle_index]["v"][j] += particle[particle_index]["a"][j];
                particle[particle_index]["p"][j] += particle[particle_index]["v"][j];
            }

            pos_stringified = get_stringified_prop(particle_index, "p");
            occupation_index = (pos_stringified in occupied) ? length(occupied[pos_stringified]) : 0;
            occupied[pos_stringified][occupation_index] = particle_index;
        }

        # remove particles at positions with more than one occupant
        for (pos_stringified in occupied) {
            if (length(occupied[pos_stringified]) > 1) {
                for (i in occupied[pos_stringified]) {
                    removable_index = occupied[pos_stringified][i];
                    delete particle[removable_index];
                }

                last_changed = round_counter;
            }
        }

        delete occupied;
    }

    print length(particle);
}
