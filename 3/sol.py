import fileinput
import re

L = [line.strip() for line in fileinput.input()]

grid = {}

for line in L:
    _, x, y, w, h = map(int,re.match(r'^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$', line).groups())
    for i in range(x+1, x+w+1):
        for j in range(y+1, y+h+1):
            key = (i,j)
            grid[key] = grid.get(key, 0) + 1

cnt = 0
for key in grid:
    if grid[key] > 1:
        cnt += 1
print(cnt)