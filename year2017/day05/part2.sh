awk '
    {field[NR-1] = $0}
    END{
        for (i = 0; i >= 0 && i < NR; ++steps) {
            next_i = i + field[i];
            field[i] += ((field[i] >= 3) ? -1 : 1);
            i = next_i;
        }
        print steps;
    }
' <input.txt
