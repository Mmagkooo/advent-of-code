#!/usr/bin/awk -f

function insert(L, i, element, j) {
    j = length(L);
    while (j > i) {
        L[j] = L[j - 1];
        --j;
    }
    L[i] = element;
}

BEGIN{
    i = 0;
    list[i] = 0;
}

{step=$0}

END{
    while (length(list) <= 2017) {
        element = length(list);
        i = (i + step) % element + 1;
        insert(list, i, element);
    }

    print list[(i + 1) % length(list)];
}