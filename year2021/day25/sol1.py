import sys

field = [line.strip() for line in sys.stdin]
HEIGHT = len(field)
WIDTH = len(field[0])
EMPTY = "."

def get_empty_field(height, width):
    return [[None for _ in range(width)] for _ in range(height)]

def move_east(field):
    moved = False
    new_field = get_empty_field(HEIGHT, WIDTH)
    for i, row in enumerate(field):
        for j, el in enumerate(row):
            if new_field[i][j]:
                continue # place already filled by moving from previous col
            next_j = (j + 1) % WIDTH
            if el == ">" and row[next_j] == EMPTY:
                new_field[i][j] = EMPTY
                new_field[i][next_j] = ">"
                moved = True
            else:
                new_field[i][j] = el
    return new_field, moved

def move_south(field):
    moved = False
    new_field = get_empty_field(HEIGHT, WIDTH)
    for i, row in enumerate(field):
        next_i = (i + 1) % HEIGHT
        for j, el in enumerate(row):
            if new_field[i][j]:
                continue # place already filled by moving from previous row
            if el == "v" and field[(i + 1) % HEIGHT][j] == EMPTY:
                new_field[i][j] = EMPTY
                new_field[next_i][j] = "v"
                moved = True
            else:
                new_field[i][j] = el
    return new_field, moved

def move(field):
    field, moved_east = move_east(field)
    field, moved_south = move_south(field)
    return field, (moved_east or moved_south)

steps = 0
something_moved = True
while something_moved:
    field, something_moved = move(field)
    steps += 1

print(steps)
