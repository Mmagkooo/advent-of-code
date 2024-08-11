#!/usr/bin/awk -f

function turn(where, new_dir) {
    new_dir = TURN[curr_dir[1], curr_dir[2], where];
    split(new_dir, curr_dir, DIR_SEP);
}

BEGIN{
    CLEAN = ".";
    WEAKENED = "W";
    INFECTED = "#";
    FLAGGED = "F";

    DIR_SEP = ", ";

    UP = "-1, 0";
    DOWN = "1, 0";
    RIGHT = "0, 1";
    LEFT = "0, -1";

    # R - right
    # L - left
    # B - back (opposite direction)

    # when UP
    TURN[-1, 0, "R"] = RIGHT;
    TURN[-1, 0, "L"] = LEFT;
    TURN[-1, 0, "B"] = DOWN;

    # when DOWN
    TURN[1, 0, "R"] = LEFT;
    TURN[1, 0, "L"] = RIGHT;
    TURN[1, 0, "B"] = UP;

    # when RIGHT
    TURN[0, 1, "R"] = DOWN;
    TURN[0, 1, "L"] = UP;
    TURN[0, 1, "B"] = LEFT;

    # when LEFT
    TURN[0, -1, "R"] = UP;
    TURN[0, -1, "L"] = DOWN;
    TURN[0, -1, "B"] = RIGHT;

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
        if (!curr_node || curr_node == CLEAN) {
            turn("L");
            grid[curr_i][curr_j] = WEAKENED;
        } else if (curr_node == WEAKENED) {
            # same direction
            grid[curr_i][curr_j] = INFECTED;
            ++new_infections;
        } else if (curr_node == INFECTED) {
            turn("R");
            grid[curr_i][curr_j] = FLAGGED;
        } else if (curr_node == FLAGGED) {
            turn("B");
            grid[curr_i][curr_j] = CLEAN;
        } else {
            print "Invalid node value:", curr_node;
            exit 2;
        }

        curr_i += curr_dir[1];
        curr_j += curr_dir[2];
    }

    print new_infections;
}