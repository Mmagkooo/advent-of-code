#!/usr/bin/awk -f

# After analyzing the opcodes, the conclusion is that the program is counting composite numbers
# from 67 * 100 + 100000 to 67 * 100 + 100000 + 17000, inclusive, but considering every 17th
# number, i.e. traversing with a step of 17. The inefficient solution is in part2-inefficient.awk

BEGIN{
    start = 67 * 100 + 100000;
    end = start + 17000;

    for (i = 2; i <= end; ++i) {
        prime[i] = 1;
    }

    for (i = 2; i <= end; ++i) {
        if (prime[i]) {
            for (j = i * 2; j <= end; j += i) {
                prime[j] = 0;
            }
        }
    }

    composite_count = 0;
    for (i = start; i <= end; i += 17) {
        composite_count += !prime[i];
    }
    print composite_count;
}
