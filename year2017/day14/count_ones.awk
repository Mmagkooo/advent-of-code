#!/usr/bin/awk -f

function count_ones(n, cnt) {
    cnt = 0;
    while (n) {
        cnt += n % 2;
        n = int(n / 2)
    }
    return cnt;
}

BEGIN{
    # define hex->dec mapping
    split("0123456789abcdef", dec_to_hex, "");
    for (dec in dec_to_hex) {
        hex = dec_to_hex[dec];
        hex_to_dec[hex] = dec - 1; # due to 1-indexing of split
    }
    HASH_LENGTH = 32;
}
{
    if (length($0) != HASH_LENGTH) {
        print "Invalid length";
        exit 1;
    }

    ones = 0;
    split($0, hex_digits, "");
    for (i = 1; i <= HASH_LENGTH; ++i) {
        new_ones = count_ones(hex_to_dec[hex_digits[i]]);
        ones += new_ones;
    }
    print ones;
}