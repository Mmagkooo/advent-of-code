#!/usr/bin/awk -f

function terminate(reason, code) {
    print reason;
    print sent_cnt[1];
    exit code;
}

# Returns the next index
function exec(process_index, command_index, op) {
    split(raw_commands[command_index], command, " ");
    op = command[1];

    if (op == "snd") {
        send(process_index, get_value(process_index, command[2]));
    } else if (op == "set") {
        registers[process_index][command[2]] = get_value(process_index, command[3]);
    } else if (op == "add") {
        registers[process_index][command[2]] += get_value(process_index, command[3]);
    } else if (op == "mul") {
        registers[process_index][command[2]] *= get_value(process_index, command[3]);
    } else if (op == "mod") {
        registers[process_index][command[2]] %= get_value(process_index, command[3]);
    } else if (op == "rcv") {
        if (length(queue[process_index])) {
            waiting[process_index] = 0;
            registers[process_index][command[2]] = receive(process_index);;
        } else {
            waiting[process_index] = 1;
            --command_index; # index should stay at the same command to check again
            if (waiting[1 - process_index]) {
                terminate("Deadlock", 0);
            }
        }
    } else if (op == "jgz") {
        if (get_value(process_index, command[2]) > 0) {
            # all other commands require incrementing the index by 1, so now subtract it
            command_index += get_value(process_index, command[3]) - 1;
        }
    } else {
        terminate("Invalid op: " op, 1);
    }

    return command_index + 1;
}

function get_value(process_index, v) {
    return (v ~ /-?[0-9]+/) ? v : registers[process_index][v];
}

function send(sender_index, value, next_index) {
    next_index = length(queue[1 - sender_index]);
    queue[1 - sender_index][next_index] = value;
    ++sent_cnt[sender_index];
}

function receive(receiver_index, last_index, i) {
    ret = queue[receiver_index][0];
    last_index = length(queue[receiver_index]) - 2;
    for (i = 0; i <= last_index; ++i) {
        # shift left by 1
        queue[receiver_index][i] = queue[receiver_index][i + 1];
    }
    delete queue[receiver_index][last_index + 1];
    return ret;
}

{raw_commands[NR - 1] = $0}

function legal_index(i) {
    return i >= 0 && i < length(raw_commands);
}

END{
    # initialize
    n_processes = 2;
    for (i = 0; i < n_processes; ++i) {
        command_index[i] = 0;
        registers[i]["p"] = i;
    }

    while (legal_index(command_index[0]) && legal_index(command_index[1])) {
        for (i = 0; i < n_processes; ++i) {
            command_index[i] = exec(i, command_index[i]);
        }
    }
    terminate("Invalid index", 0);
}
