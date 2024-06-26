awk '
    {field[NR-1] = $0}
    END{
        for (i = 0; i >= 0 && i < NR; i += field[i]++) {
            ++steps;
        }
        print steps;
    }
' <input.txt
