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

output = []
change = True
while True:
    if not code2op: break
    change = False
    mincode = min(code2op, key=lambda code: len(code2op[code]))
    ops = code2op[mincode]
    if len(ops) != 1:
        #print("err", mincode, ops)
        break
        #exit()
    minop = ops.pop()
    output.append((mincode, minop))
    #print(mincode, minop)
    del code2op[mincode]
    del op2code[minop]
    
    '''
    for op in op2code:
        codes = op2code[op]
        if mincode in codes:
            codes.remove(mincode)
            change = True
    '''
    for code in code2op:
        ops = code2op[code]
        if minop in ops:
            ops.remove(minop)
            change = True
    
'''    
change = True
while True:
    if not op2code: break
    change = False
    minop = min(op2code, key=lambda op: len(op2code[op]))
    codes = op2code[minop]
    if len(codes) != 1:
        #print("err", minop, codes)
        break
        #exit()
    mincode = codes.pop()
    print(minop, mincode)
    del op2code[minop]
    del code2op[mincode]
    
    for op in op2code:
        codes = op2code[op]
        if mincode in codes:
            codes.remove(mincode)
            change = True'''
'''        
    for code in code2op:
        ops = code2op[code]
        if minop in ops:
            ops.remove(minop)
            change = True
'''
#for code in code2op:
#    print(code, sorted(code2op[code]))
#print()
#for op in op2code:
#    print(op, sorted(op2code[op]))

print(list(map(lambda x: x[1], sorted(output))))