import sys
import re

R = re.compile(r'^position=<(.*),(.*)> velocity=<(.*),(.*)>$')

INF = 1e9

m = {} # (x,y) -> [(vx, vy),...]
        
def pretty():
    minx = miny = INF
    maxx = maxy = -INF
    for x, y in m:
        maxx = max(x, maxx)
        maxy = max(y, maxy)
        minx = min(x, minx)
        miny = min(y, miny)
        
    for y in range(miny, maxy+1):
        for x in range(minx, maxx+1):
            print("#" if (x,y) in m else ".", end="")
        print()
    
adj = [(-1,-1), (0,-1), (0,1), (-1,0), (1,0), (-1,1), (0,1), (1,1)]
def single_alone(x, y):
    alone = True
    for dx, dy in adj:
        np = (x+dx, y+dy)
        if np in m:
            alone = False
            break
    return alone
    
def any_alone():
    for x, y in m:
        if single_alone(x, y):
            return True
    return False

def move():
    global m
    nm = {}
    for x,y in m:
        l = m[x,y]
        for vx, vy in l:
            np = (x+vx, y+vy)
            if np not in nm:
                nm[np]= []
            nm[np].append((vx, vy))
    m = nm
        
f = open(sys.argv[1], "r")
        
for line in f.readlines():
    x, y, vx, vy = map(int, re.match(R, line).groups())
    p = (x,y)
    if p not in m:
        m[p] = []
    m[p].append((vx,vy))
    
f.close()
    
t = 0
while True:
    if not any_alone():
        pretty()
        input("t = {}, Press a key to continue".format(t))
    move()
    t += 1