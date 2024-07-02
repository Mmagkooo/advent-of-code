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

function count_garbage(chars, i, cnt) {
    cnt = 0;
    for (i = 1; i <= length(chars); ++i) {
        if (chars[i] == "<") {
            ++i; # not counting <
            for (; chars[i] != ">"; ++i) {
                ++cnt;
            }
        }
    }
    return cnt;
}

{
    split($0, chars, "");
    remove_exclamation(chars);
    sol = count_garbage(chars);
    print sol;
}
