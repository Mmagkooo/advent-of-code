import re
from operators import *

R = re.compile(r'^.*\[(\d*), (\d*), (\d*), (\d*)\]$')

f = open("input.txt")
lines = [line.strip() for line in f.readlines() if line.strip()]
f.close()

m = {}

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
    
    if code not in m:
        m[code] = []
    m[code].append((iline, before, after))
    
code2op = {}
for code in m:
    for operator in operators:
        can_be = True
        for iline, before, after in m[code]:
            if operator(before, iline) != after:
                can_be = False
                break
        
        if can_be:
            if code not in code2op:
                code2op[code] = set()
            code2op[code].add(operator.__name__)

op2code = {}
for operator in operators:
    for code in m:
        can_be = True
        for iline, before, after in m[code]:
            if operator(before, iline) != after:
                can_be = False
                break
        
        if can_be:
            if operator.__name__ not in op2code:
                op2code[operator.__name__] = set()
            op2code[operator.__name__].add(code)

seen = set()
def match_left(code):
    seen.add(code)
    for op in code2op[code]:
        if op in seen:
            continue
        nmatch = match_right(op, ?)
        if len(nmatch) % 2 == 0:
            nmatch.add((code, op))
            return nmatch
    return set()

def match_right(op, dist):
    seen.add(op)
    for code in op2code[op]:
        if code in seen:
            continue
        nmatch = match_left(code)
        if len(nmatch) % 2 == 1:
            nmatch.add((code, op))
            return nmatch
    return set()
            
            
all_edges = set()
for code in code2op:
    for op in code2op[code]:
        all_edges.add((code, op))
            
matching = set()
found = True
while found:
    found = False
    for code in code2op:
        ops = code2op[code]
        if len(ops) == 1:
            nmatch = match_left(code)
            if not nmatch:
                continue
            
            
            
            found = True
            break