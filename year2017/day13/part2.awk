#!/usr/bin/awk -f

BEGIN{FS=": "}
{
    depths[NR - 1] = $1;
    ranges[NR - 1] = $2;
}
END{
    for (delay = 0; ; ++delay) {
        severity = 0;
        for (i = 0; i < NR; ++i) {
            if ((depths[i] + delay) % (2 * (ranges[i] - 1)) == 0) {
                severity = 1;
                break;
            }
        }
    
        if (!severity) {
            print delay;
            break;
        }
    }
}
