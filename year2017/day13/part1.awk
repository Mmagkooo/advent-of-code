#!/usr/bin/awk -f

BEGIN{FS=": "}
{
    depth = $1;
    range = $2;

    # Every scanner has period 2 * (range - 1).
    # The packet visits a scanner's layer at moment equal (in ps) to the scanner's depth.
    if (depth % (2 * (range - 1)) == 0) {
        severity += depth * range;
    }
}
END{print severity}
