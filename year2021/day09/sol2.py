import sys

lines = [line.strip() for line in sys.stdin]

height = {}
for i, line in enumerate(lines):
    for j, val in enumerate(line):
        height[i, j] = int(val)

INF = 10
diffs = (-1, 0), (1, 0), (0, -1), (0, 1)

low_points = []
for i in range(len(lines)):
    for j in range(len(lines[i])):
        current_height = height[i, j]
        for di, dj in diffs:
            adj_height = height.get((i+di, j+dj), INF)
            if current_height >= adj_height:
                break
        else:
            low_points.append((i, j))

def populate_basin(point, basin):
    if point in basin:
        return
    basin.add(point)

    current_height = height[point]
    i, j = point

    for di, dj in diffs:
        adj = (i + di, j + dj)
        if adj not in height:
            continue

        adj_height = height[adj]
        if adj_height == 9:
            continue

        populate_basin(adj, basin)

basin = {}
for low_point in low_points:
    basin[low_point] = set()
    populate_basin(low_point, basin[low_point])

TOP_N = 3
basin_sizes = map(lambda low_point: len(basin[low_point]), low_points)
sol = 1
for size in sorted(basin_sizes)[-TOP_N:]:
    sol *= size
print(sol)
