DIM = 300
MDIM = 3
ID = 42


def f(x, y):
    return ((x+10)*y+ID)*(x+10) % 1000 // 100 - 5

def pretty(m):
    for y in range(len(m)):
        for x in range(len(m[y])):
            print("{:3}".format(m[y][x]), end="")
        print()
    
m = []
for y in range(1, DIM+1):
    row = []
    for x in range(1, DIM+1):
        row.append(f(x, y))
    m.append(row)

maxs = 0
maxp = None
for y in range(1, DIM-MDIM+1):
    for x in range(1, DIM-MDIM+1):
        s = 0
        for dy in range(MDIM):
            for dx in range(MDIM):
                s += m[y+dy-1][x+dx-1]
        if s > maxs:
            maxs = s
            maxp = (x, y)

print(maxp, maxs)