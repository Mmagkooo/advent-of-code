#!/usr/bin/awk -f

BEGIN{
    hex_to_bin["0"] = "0000";
    hex_to_bin["1"] = "0001";
    hex_to_bin["2"] = "0010";
    hex_to_bin["3"] = "0011";
    hex_to_bin["4"] = "0100";
    hex_to_bin["5"] = "0101";
    hex_to_bin["6"] = "0110";
    hex_to_bin["7"] = "0111";
    hex_to_bin["8"] = "1000";
    hex_to_bin["9"] = "1001";
    hex_to_bin["a"] = "1010";
    hex_to_bin["b"] = "1011";
    hex_to_bin["c"] = "1100";
    hex_to_bin["d"] = "1101";
    hex_to_bin["e"] = "1110";
    hex_to_bin["f"] = "1111";
}
{
    split($0, hex_digits, "");
    for (i = 1; i <= length(hex_digits); ++i) {
        printf hex_to_bin[hex_digits[i]];
    }
    printf "\n";
}