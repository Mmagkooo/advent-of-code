#!/usr/bin/awk -f

# ./part1.awk [-v PROGRAMS=<STR>] [FILENAME]
# omitting the vars yields the task result

function shift_left(arr, start_index, amount, initial_length) {
    initial_length = length(arr);
    for (i = start_index; i < start_index + initial_length; ++i) {
        arr[i - amount] = arr[i];
    }

    for (i = start_index + initial_length - amount; i < start_index + initial_length; ++i) {
        delete arr[i];
    }
}

BEGIN{
    RS=",";
    FIELDWIDTHS="1 *";
    PROGRAMS_INPUT = PROGRAMS ? PROGRAMS : "abcdefghijklmnop";
    split(PROGRAMS_INPUT, programs, "");
    N_PROGRAMS = length(programs);
    shift_left(programs, 1, 1);
}

$1 == "s" {
    # Rather complicated but works:
    # Append the first (N_PROGRAMS - shift_amount) programs to the end of the array;
    # Then shift the whole array to the left to overwrite what was copied.
    shift_amount = $2;
    for (i = 0; i < N_PROGRAMS - shift_amount; ++i) {
        programs[N_PROGRAMS + i] = programs[i];
    }

    shift_left(programs, N_PROGRAMS - shift_amount, N_PROGRAMS - shift_amount);
}

{split($2, args, "/")}

$1 == "x" {
    tmp = programs[args[1]];
    programs[args[1]] = programs[args[2]];
    programs[args[2]] = tmp;
}

$1 == "p" {
    for (position in programs) {
        if (programs[position] == args[1]) {
            programs[position] = args[2];
        } else if (programs[position] == args[2]) {
            programs[position] = args[1];
        }
    }
}

END{
    for (i = 0; i < N_PROGRAMS; ++i) {
        printf programs[i];
    }
    printf "\n";
}