awk '
    {
        min=max=$1;
        for (i=2; i<=NF; ++i) {
            if ($i < min) min = $i;
            if ($i > max) max = $i;
        }
        sol += (max - min);
    }
    END{print sol}
' input.txt
