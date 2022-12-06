import sys

lines = [line.strip() for line in sys.stdin]

height = {}
for i, line in enumerate(lines):
    for j, val in enumerate(line):
        height[i, j] = int(val)

INF = 10

sol = 0
for i in range(len(lines)):
    for j in range(len(lines[i])):
        current_height = height[i, j]
        diffs = (-1, 0), (1, 0), (0, -1), (0, 1)
        for di, dj in diffs:
            adj_height = height.get((i+di, j+dj), INF)
            if current_height >= adj_height:
                break
        else:
            sol += current_height + 1

print(sol)
