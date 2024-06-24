set -eu

echo "$1" | awk '
    function sum_adjacent(x, y, storage) {
        sum = 0;
        for (dx = -1; dx <= 1; ++dx) {
            for (dy = -1; dy  <= 1; ++dy) {
                if (dx == 0 && dy == 0) continue;
                sum += storage[x + dx, y + dy];
            }
        }
        return sum;
    }
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
                    storage[x, y] = number;
                    if (number > end_number) {
                        next;
                    }

                    x += direction_x[direction_index];
                    y += direction_y[direction_index];
                    number = sum_adjacent(x, y, storage);
                    storage[x, y] = number;
                }
                direction_index = (direction_index + 1) % direction_count;
            }
        }
    }
    END{print number}
'
