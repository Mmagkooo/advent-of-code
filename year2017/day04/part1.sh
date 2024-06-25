awk '
    {
        delete seen;
        for (i=1; i<=NF; ++i) {
            if (seen[$i]++) {
                next;
            }
        };
        sol += 1;
    }
    END{print sol}
' <input.txt
