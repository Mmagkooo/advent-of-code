awk '
    {
        for (i=1; i<=NF; ++i) {
            for (j=1; j<=NF; ++j) {
                if (i == j) continue;
                if ($i % $j == 0) {
                    sol += $i / $j;
                    next;
                }
            }
        }
    }
    END{print sol}
' input.txt
