import re
from operators import *

R = re.compile(r'^.*\[(\d*), (\d*), (\d*), (\d*)\]$')

f = open("input.txt")
lines = [line.strip() for line in f.readlines() if line.strip()]
f.close()

m = {}

global_cnt = 0
i = 0
while i < len(lines):
    bline = lines[i]
    iline = lines[i+1]
    aline = lines[i+2]
    i += 3
    before = tuple(map(int, re.match(R, bline).groups()))
    iline = tuple(map(int, iline.split()))
    after = tuple(map(int, re.match(R, aline).groups()))
    code = iline[0]
    iline = iline[1:]
    
    cnt = 0
    for op in operators:
        if op(before, iline) == after:
            cnt += 1
        if cnt == 3:
            break
    
    if cnt >= 3:
        global_cnt += 1
        
print(global_cnt)