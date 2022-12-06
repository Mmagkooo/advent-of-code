import sys

octopi = {}
for i, line in enumerate(sys.stdin):
    for j, octopus in enumerate(line.strip()):
        octopi[i, j] = int(octopus)

diffs = []
for di in range(-1, 1+1):
    for dj in range(-1, 1+1):
        if di == 0 and dj == 0:
            continue
        diffs.append((di, dj))
assert len(diffs) == 8

def spread(octopi, stack):
    while stack:
        flash_i, flash_j = stack.pop()
        for di, dj in diffs:
            adj = (flash_i + di, flash_j + dj)
            if adj in octopi:
                octopi[adj] += 1
                adj_amount = octopi[adj]
                if adj_amount == 10:
                    stack.append(adj)

steps = 0
while not all(map(lambda val: val == 0, octopi.values())):
    stack = []
    for i, j in octopi:
        octopi[i, j] += 1
        amount = octopi[i, j]
        assert amount <= 10
        if amount == 10:
            stack.append((i, j))

    spread(octopi, stack)

    for i, j in octopi:
        amount = octopi[i, j]
        if amount > 9:
            octopi[i, j] = 0

    steps += 1

print(steps)
