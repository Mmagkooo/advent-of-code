"""
The program expects one integer input parameter N. It then finds all fields located
(using Manhattan distance) N fields away from the field labelled with S.
The program outputs the input content, labelling the located fields with x.

This is merely a helper program to check certain outputs of the main Rust program.
"""

import sys
from collections import deque
from typing import Tuple, List, Set


def find_max_dist_locations(start_location: Tuple, max_dist: int, grid: List) -> Set:
    """Returns max dist - bfs"""
    max_dist_locations = set()

    seen = set()

    height = len(grid)
    width = len(grid[0])

    queue = deque([(start_location, 0)])
    while queue:
        current_state = current_loc, dist = queue.popleft()

        if (
            current_loc[0] < 0
            or current_loc[0] >= height
            or current_loc[1] < 0
            or current_loc[1] >= width
        ):
            continue

        if grid[current_loc[0]][current_loc[1]] == "#":
            continue

        if current_state in seen:
            continue
        seen.add(current_state)

        if dist == max_dist:
            max_dist_locations.add(current_loc)
            continue

        for di, dj in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            next_loc = current_loc[0] + di, current_loc[1] + dj
            queue.append((next_loc, dist + 1))

    return max_dist_locations


def find_start_location(grid: List) -> Tuple:
    """Finds location labelled with the start symbol"""
    for i, row in enumerate(grid):
        for j, c in enumerate(row):
            if c == "S":
                return i, j
    raise ValueError("Invalid grid: no start")


def main():
    """main method"""

    grid = [line.strip() for line in sys.stdin]

    start_location = find_start_location(grid)
    max_dist = int(sys.argv[1])
    max_dist_locations = find_max_dist_locations(start_location, max_dist, grid)
    for i, row in enumerate(grid):
        for j, c in enumerate(row):
            print(c if (i, j) not in max_dist_locations else "x", end="")
        print()


if __name__ == "__main__":
    main()
