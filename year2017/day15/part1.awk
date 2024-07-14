#!/usr/bin/awk -f

# only generate the interesting ending
function dec_to_bin_suffix(n, bin, digit) {
    bin = "";
    while (n && length(bin) < SUFFIX_LENGTH) {
        bin = (n % 2) bin;
        n = int(n / 2);
    }

    while (length(bin) < SUFFIX_LENGTH) {
        bin = "0" bin;
    }

    return bin;
}

BEGIN{
    FACTOR_A = 16807;
    FACTOR_B = 48271;
    MOD = 2147483647;
    SUFFIX_LENGTH = 16;
}

$2=="A"{initial_a = $NF}
$2=="B"{initial_b = $NF}

END{
    current_a = initial_a;
    current_b = initial_b;

    matches = 0;
    total_pairs = 40e6;
    while (total_pairs--) {
        current_a = (current_a * FACTOR_A) % MOD;
        current_b = (current_b * FACTOR_B) % MOD;

        matches += (dec_to_bin_suffix(current_a) == dec_to_bin_suffix(current_b));
    }

    print matches;
}