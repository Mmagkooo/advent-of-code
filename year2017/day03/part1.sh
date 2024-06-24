set -eu

echo "$1" | awk '
    function abs(a) { return a < 0 ? -a : a }
    {
        end_number = $0;
        direction_count = 4;

        # right
        direction_x[0] = 1;
        direction_y[0] = 0;

        # up
        direction_x[1] = 0;
        direction_y[1] = 1;

        # left
        direction_x[2] = -1;
        direction_y[2] = 0;

        # down
        direction_x[3] = 0;
        direction_y[3] = -1;

        # starting data
        x = 0;
        y = 0;
        direction_index = 0;
        number = 1;
        for (step_count = 1; 1; step_count += 1) {
            for (j = 1; j <= 2; ++j) {
                for (step_i = 1; step_i <= step_count; ++step_i) {
                    if (number == end_number) {
                        next;
                    }

                    x += direction_x[direction_index];
                    y += direction_y[direction_index];
                    number += 1;
                }
                direction_index = (direction_index + 1) % direction_count;
            }
        }
    }
    END{print abs(x) + abs(y)}
'
