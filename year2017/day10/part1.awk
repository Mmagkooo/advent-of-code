#!/usr/bin/awk -f

BEGIN{
    RS=",";
    size=256;
    for (i = 0; i < size; ++i) list[i] = i;
    position=0;
}

{
    current_length = $0;
    end = (position + current_length - 1 + size) % size;

    i = position;
    j = end;
    steps = int(current_length / 2);
    while (steps--) {
        tmp = list[i];
        list[i] = list[j];
        list[j] = tmp;

        i = (i + 1 + size) % size;
        j = (j - 1 + size) % size;
    }

    position = (end + 1 + skip_size++ + size) % size;
}
END{
    print list[0] * list[1];
}
