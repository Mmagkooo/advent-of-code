import sys

sys.setrecursionlimit(10000+100)

field = [[int(el) for el in line.strip()] for line in sys.stdin]
HEIGHT = len(field)
WIDTH = len(field[0])

memo = {}

def rek(i, j, so_far):
    if (i, j) in memo:
        previous_here = memo[i, j]
        if previous_here <= so_far:
            return
    
    memo[i, j] = so_far

    for di, dj in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
        ni, nj = i + di, j + dj
        if ni >= 0 and nj >= 0 and ni < HEIGHT and nj < WIDTH:
            rek(ni, nj, so_far + field[ni][nj])

rek(0, 0, 0)
print(memo[HEIGHT-1, WIDTH-1])
