#!/usr/bin/awk -f

BEGIN{
    UP = "-1 0";
    DOWN = "1 0";
    LEFT = "0 -1";
    RIGHT = "0 1";
    EMPTY_FIELD = " ";
}

{
    split($0, line, "");
    for (i = 1; i < length(line); ++i) {
        grid[NR - 1][i - 1] = line[i];
    }
}

function find_start_j(j) {
    for (j = 0; j < width; ++j) {
        if (grid[0][j] == "|") {
            return j;
        }
    }
}

# operates on global variables di and dj
function turn(i, j, direction_string) {
    direction_string = di " " dj;
    if (direction_string == UP || direction_string == DOWN) {
        if (grid[i][j - 1] != EMPTY_FIELD) {
            di = 0;
            dj = -1;
        } else if (grid[i][j + 1] != EMPTY_FIELD) {
            di = 0;
            dj = 1;
        } else {
            print "Cannot turn left or right";
            exit 1;
        }
    } else if (direction_string == LEFT || direction_string == RIGHT) {
        if (grid[i - 1][j] != EMPTY_FIELD) {
            di = -1;
            dj = 0;
        } else if (grid[i + 1][j] != EMPTY_FIELD) {
            di = 1;
            dj = 0;
        } else {
            print "Cannot turn up or down";
            exit 1;
        }
    } else {
        print "Error in turning for", direction_string;
        exit 1;
    }
}

END{
    height = NR;
    width = length(line);

    i = 0;
    j = find_start_j();
    curr_char = grid[i][j];

    # direction
    di = 1;
    dj = 0;

    steps = 0;
    while (curr_char != EMPTY_FIELD) {
        if (curr_char == "+") {
            turn(i, j);
        }

        i += di;
        j += dj;
        ++steps;
        curr_char = grid[i][j];
    }

    print steps;
}
