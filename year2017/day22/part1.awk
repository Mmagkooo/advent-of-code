#!/usr/bin/awk -f

function turn(where, new_dir) {
    new_dir = TURN[curr_dir[1], curr_dir[2], where];
    split(new_dir, curr_dir, DIR_SEP);
}

BEGIN{
    INFECTED = "#";
    CLEAN = ".";

    DIR_SEP = ", ";

    UP = "-1, 0";
    DOWN = "1, 0";
    RIGHT = "0, 1";
    LEFT = "0, -1";

    # when UP
    TURN[-1, 0, "R"] = RIGHT;
    TURN[-1, 0, "L"] = LEFT;

    # when DOWN
    TURN[1, 0, "R"] = LEFT;
    TURN[1, 0, "L"] = RIGHT;

    # when RIGHT
    TURN[0, 1, "R"] = DOWN;
    TURN[0, 1, "L"] = UP;

    # when LEFT
    TURN[0, -1, "R"] = UP;
    TURN[0, -1, "L"] = DOWN;

    if (!steps) {
        print "Specify variable 'steps'";
        exit 1;
    }
}

{split($0, grid[NR], "")}

END{
    WIDTH = length($0);
    HEIGHT = NR;

    curr_i = (HEIGHT + 1) / 2;
    curr_j = (WIDTH + 1) / 2;

    split(UP, curr_dir, DIR_SEP);

    new_infections = 0;

    while (steps--) {
        curr_node = grid[curr_i][curr_j];
        if (curr_node == INFECTED) {
            turn("R");
            grid[curr_i][curr_j] = CLEAN;
        } else {
            turn("L");
            grid[curr_i][curr_j] = INFECTED;
            ++new_infections;
        }
        curr_i += curr_dir[1];
        curr_j += curr_dir[2];
    }

    print new_infections;
}