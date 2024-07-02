#!/usr/bin/awk -f

function remove_exclamation(chars, i, j, orig_len) {
    j = 1;
    orig_len = length(chars);

    for (i = 1; i <= orig_len; ++i) {
        if (chars[i] == "!") {
            ++i;
        } else {
            chars[j++] = chars[i];
        }
    }

    # remove remaining chars
    for (; j <= orig_len; ++j) {
        delete chars[j];
    }
}

function remove_garbage(chars, i, j, orig_len) {
    j = 1;
    orig_len = length(chars);

    for (i = 1; i <= orig_len; ++i) {
        if (chars[i] == "<") {
            for (; chars[i] != ">"; ++i) {}
        } else {
            chars[j++] = chars[i];
        }
    }

    # remove remaining chars
    for (; j <= orig_len; ++j) {
        delete chars[j];
    }
}

function count(chars, i, sol, depth) {
    for (i = 1; i <= length(chars); ++i) {
        if (chars[i] == "{") {
            depth++;
        } else if (chars[i] == "}") {
            sol += depth;
            --depth;
        }
    }

    return sol;
}

{
    split($0, chars, "");
    remove_exclamation(chars);
    remove_garbage(chars);
    sol = count(chars);
    print sol;
}
