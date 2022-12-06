import sys

INF = 1e9
LIM = 10000

def manhattan(p1, p2):
    return abs(p1[0]-p2[0]) + abs(p1[1]-p2[1])

L = []
minx, maxx, miny, maxy = INF, -INF, INF, -INF
p2i = {}
for i, line in enumerate(sys.stdin):
    x,y = map(int, line.split(", "))
    minx = min(minx, x)
    maxx = max(maxx, x)
    miny = min(miny, y)
    maxy = max(maxy, y)
    L.append((x, y))

cnter = 0
    
for x in range(minx-1, maxx+2):
    for y in range(miny-1, maxy+2):
        suma = 0
        for i, p in enumerate(L):
            suma += manhattan((x, y), p)
        if suma < LIM:
            cnter += 1
            
print(cnter)