DIM = 300
#MDIM = 3
ID = 2187


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

#pretty(m)
    
maxs = -1e9
maxp = None
for y in range(1, DIM+1):
    print("DEBUG y =", y)
    for x in range(1, DIM+1):
        MINDIM = min(DIM-x, DIM-y)+1
        prev = 0
        for MDIM in range(1, MINDIM+1):
            s = prev
            for dy in range(MDIM):
                s += m[y+dy-1][x+MDIM-2]
            for dx in range(MDIM):
                s += m[y+MDIM-2][x+dx-1]
            s -= m[y+MDIM-2][x+MDIM-2]
            prev = s
            if s > maxs:
                maxs = s
                maxp = (x, y, MDIM)

print(maxp, maxs)