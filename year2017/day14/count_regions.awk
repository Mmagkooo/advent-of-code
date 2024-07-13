#!/usr/bin/awk -f

function flood_fill(i, j) {
    if (i < 1 || i > GRID_SIZE || j < 1 || j > GRID_SIZE) {
        return;
    }

    if (!grid[i,j]) {
        return;
    }

    if (seen[i,j]) {
        return;
    }
    seen[i,j] = 1;

    flood_fill(i - 1, j);
    flood_fill(i + 1, j);
    flood_fill(i, j - 1);
    flood_fill(i, j + 1);
}

BEGIN{
    GRID_SIZE = 128;
}

{
    split($0, chars, "");
    for (i = 1; i <= length(chars); ++i) {
        grid[NR,i] = chars[i];
    }
}

END{
    regions = 0;
    for (i = 1; i <= GRID_SIZE; ++i) {
        for (j = 1; j <= GRID_SIZE; ++j) {
            if (grid[i,j] && !seen[i,j]) {
                ++regions;
                flood_fill(i, j);
            }
        }
    }
    print regions;
}