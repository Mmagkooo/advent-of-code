#!/usr/bin/awk -f

# Returns the next index
function exec(command_index, op) {
    split(raw_commands[command_index], command, " ");
    op = command[1];

    if (op == "set") {
        registers[command[2]] = get_value(command[3]);
    } else if (op == "sub") {
        registers[command[2]] -= get_value(command[3]);
    } else if (op == "mul") {
        registers[command[2]] *= get_value(command[3]);
        ++mul_counter;
    } else if (op == "jnz") {
        if (get_value(command[2]) != 0) {
            # all other commands require incrementing the index by 1, so now subtract it
            command_index += get_value(command[3]) - 1;
        }
    } else {
        print "Invalid op:", op;
        exit 1;
    }

    return command_index + 1;
}

function get_value(v) {
    return (v ~ /-?[0-9]+/) ? v : registers[v];
}

BEGIN{
    mul_counter = 0;
}

{raw_commands[NR - 1] = $0}

END{
    command_index = 0;
    while (command_index >= 0 && command_index < length(raw_commands)) {
        command_index = exec(command_index);
    }

    print mul_counter;
}
