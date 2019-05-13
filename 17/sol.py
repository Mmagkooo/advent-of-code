import sys
import re

INF = int(1e9)
R = re.compile(r"^(x|y)=(\d+), (x|y)=(\d+)\.\.(\d+)$")

spring_x = 500
spring_y = 0

m = {}
xmin = ymin = INF
xmax = ymax = -INF

def pretty():
    for y in range(0, ymax+1):
        for x in range(0, xmax+1):
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

 
def drop(curr_y, curr_x):
    while (curr_y+1, curr_x) not in m and curr_y <= ymax:
        curr_y += 1
    
    if curr_y > ymax:
        return
        
    lcurr_x = curr_x
    while (curr_y, lcurr_x-1) not in m and (curr_y+1, lcurr_x) in m and lcurr_x-1 >= xmin:
        lcurr_x -= 1
        
    if (curr_y+1, lcurr_x) not in m:
        drop(curr_y+1, lcurr_x)
    elif (curr_y, lcurr_x-1) in m:
        for x in range(lcurr_x, curr_x):
            m[curr_y, x] = '|'
        
    ########
    rcurr_x = curr_x
    while (curr_y, rcurr_x+1) not in m and (curr_y+1, rcurr_x) in m and rcurr_x+1 <= xmax:
        rcurr_x += 1
        
    if (curr_y+1, rcurr_x) not in m:
        drop(curr_y+1, rcurr_x)
    elif (curr_y, rcurr_x+1) in m:
        for x in range(curr_x+1, rcurr_x+1):
            m[curr_y, x] = '|'
    
    ########
    if (curr_y, curr_x-1) in m and (curr_y, curr_x+1) in m:
        for x in range(lcurr_x, rcurr_x+1):
            m[curr_y, x] = 'X'

def finish(curr_y, curr_x):
    while (curr_y+1, curr_x) not in m and curr_y <= ymax:
        m[curr_y, curr_x] = '|'
        curr_y += 1
        
    if curr_y > ymax:
        return
        
    if m[curr_y+1, curr_x] == '|':
        return
        
    m[curr_y, curr_x] = '|'
        
    lcurr_x = curr_x-1
    while (curr_y, lcurr_x-1) not in m and (curr_y+1, lcurr_x) in m and lcurr_x-1 >= xmin:
        m[curr_y, lcurr_x] = '|'
        lcurr_x -= 1
        
    if (curr_y+1, lcurr_x) not in m:
        m[curr_y, lcurr_x] = '|'
        finish(curr_y+1, lcurr_x)
        
    rcurr_x = curr_x+1
    while (curr_y, rcurr_x+1) not in m and (curr_y+1, rcurr_x) in m and rcurr_x+1 <= xmax:
        m[curr_y, rcurr_x] = '|'
        rcurr_x += 1
        
    if (curr_y+1, rcurr_x) not in m:
        m[curr_y, rcurr_x] = '|'
        finish(curr_y+1, rcurr_x)
        
while True:
#    pretty()
    old_cnt = len(m)
    drop(spring_y+1, spring_x)
    new_cnt = len(m)
    #if new_cnt-old_cnt == 2574:
        #finish(spring_y+1, spring_x)
        #pretty()
        #exit()
    if new_cnt == old_cnt:
        print("STOP")
        break

finish(spring_y+1, spring_x)
pretty()
suma = 0
for key in m:
    if m[key] == 'X' or m[key] == '|': suma += 1
print(suma)