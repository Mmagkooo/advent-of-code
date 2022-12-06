import fileinput
import re

L = [line.strip() for line in fileinput.input()]

grid = {}

for line in L:
    ID, x, y, w, h = map(int,re.match(r'^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$', line).groups())
    for i in range(x+1, x+w+1):
        for j in range(y+1, y+h+1):
            key = (i,j)
            if key not in grid:
                grid[key] = []
            grid[key].append(ID)

            
overlappers = set()
for key in grid:
    IDs = grid[key]
    if len(IDs) > 1:
        for ID in IDs:
            overlappers.add(ID)
        
        
for i in range(1, len(L)+1):
    if i not in overlappers:
        print(i)