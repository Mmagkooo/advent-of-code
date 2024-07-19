#!/usr/bin/awk -f

BEGIN{
    i = 0;
    element = 0;
}

{step=$0}

END{
    # value 0 is always at index 0; it is sufficient to track when i = 1
    after_0 = -1;
    while (++element <= 50e6) {
        i = (i + step) % element + 1;
        if (i == 1) {
            after_0 = element;
        }
    }

    print after_0;
}
