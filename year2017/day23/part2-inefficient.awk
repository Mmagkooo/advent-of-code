#!/usr/bin/awk -f

# The efficient solution is in part2.awk

BEGIN{
    a = b = c = d = e = f = g = h = 0;
    a = 1;  # modification for part 2
    b = c = 67;

    if (a != 0) {
        b = b * 100 + 100000;
        c = b + 17000;
    }

    for (; b <= c; b += 17) {
        f = 1;
        for (d = 2; d <= b; ++d) {
            for (e = 2; e <= b; ++e) {
                if (d * e == b) {
                    f = 0;
                }
            }
        }

        if (f == 0) {
            ++h;
        }
    }

    print h;
}
