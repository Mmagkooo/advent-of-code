import sys
import re

INF = int(1e9)
R = re.compile(r"^(x|y)=(\d+), (x|y)=(\d+)\.\.(\d+)$")
FELL = "fell"
BLOCKED = "blocked"

spring_x = 500
spring_y = 0

m = {}
xmin = ymin = INF
xmax = ymax = -INF

def pretty():
    for y in range(0, ymax+1):
        for x in range(xmin, xmax+1+1):
            if x == spring_x and y == spring_y:
                print('+', end="")
            elif (y,x) in drop.tmp:
                print('|', end="")
            else:
                print(m.get((y,x), '.'), end="")
        print()
    print()

for line in sys.stdin:
    static, val, dynamic, start, end = R.match(line.strip()).groups()
    val = int(val)
    start = int(start)
    end = int(end)
    
    if static == 'x':
        xmin = min(xmin, val)
        xmax = max(xmax, val)
        ymin = min(ymin, start)
        ymax = max(ymax, end)
        for y in range(start, end+1):
            m[y, val] = '#'
            
    elif static == 'y':
        xmin = min(xmin, start)
        xmax = max(xmax, end)
        ymin = min(ymin, val)
        ymax = max(ymax, val)
        for x in range(start, end+1):
            m[val, x] = '#'
    else:
        raise ValueError("illegal static: " + static)

clay_cnt = len(m)

def drop(y, x):
    while y <= ymax and (y,x) not in m and (y,x) not in drop.tmp:
        drop.tmp.add((y,x))
        y += 1
    
    if (y,x) in drop.tmp: return
    
    if y >= ymax and (y,x) not in m: return
    
    y -= 1
    
    lx = x-1
    #left
    while (y,lx) not in m and (y,lx) not in drop.tmp and (y+1,lx) in m:
        drop.tmp.add((y,lx))
        lx -= 1
    
    if (y,lx) not in m and (y+1,lx) not in m:
        drop(y, lx)
    
    rx = x+1
    #right
    while (y,rx) not in m and (y,rx) not in drop.tmp and (y+1,rx) in m:
        drop.tmp.add((y,rx))
        rx += 1
        
    if (y,rx) not in m and (y+1,rx) not in m:
        drop(y,rx)
    
    if (y,lx) in m and (y,rx) in m:
        for tx in range(lx+1, rx):
            m[y,tx] = '~'

while True:
    old_cnt = len(m)
    drop.tmp = set()
    drop(spring_y+1, spring_x)
    diff = len(m) - old_cnt
    if diff == 0:
        print(len(m) + len(drop.tmp) - clay_cnt)
        print(len(m) - clay_cnt)
        break
pretty()