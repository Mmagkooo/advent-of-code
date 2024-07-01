#!/usr/bin/awk -f

function condition(reg, op, val) {
    if (op == "==") {
        return regs[reg] == val;
    } else if (op == ">") {
        return regs[reg] > val;
    } else if (op == ">=") {
        return regs[reg] >= val;
    } else if (op == "<") {
        return regs[reg] < val;
    } else if (op == "<="){
        return regs[reg] <= val;
    } else if (op == "!=") {
        return regs[reg] != val;
    } else {
        print "Invalid cond operator", op;
        exit 1;
    }
}

function exec(reg, op, val)  {
    if (op == "inc") {
        regs[reg] += val;
    } else if (op == "dec") {
        regs[reg] -= val;
    } else {
        print "Invalid exec operator", op;
        exit 2;
    }
}

{
    if (condition($5, $6, $7)) {
        exec($1, $2, $3);
    }
}
END{
    max_val = -1000000
    for (name in regs) {
        curr_val = regs[name];
        if (curr_val > max_val) {
            max_val = curr_val;
        }
    }
    print max_val;
}