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

    return regs[reg];
}

{
    if (condition($5, $6, $7)) {
        new_val = exec($1, $2, $3);
        if (new_val > max_val) {
            max_val = new_val;
        }
    }
}
END{print max_val}