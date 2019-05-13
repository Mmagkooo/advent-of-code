import re
from operators import *
import sys

R = re.compile(r'^.*\[(\d*), (\d*), (\d*), (\d*)\]$')

ops = [eqri, mulr, gtri, gtrr, banr, addi, seti, gtir, muli, bori, setr, addr, bani, borr, eqir, eqrr]
r = [0, 0, 0, 0]

for line in sys.stdin:
    code, a, b, c = map(int, line.split())
    ins = a, b, c
    r = ops[code](r, ins)
    print(r)

'''
lines = [line.strip() for line in sys.stdin if line.strip()]
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
    
    if ops[code](before, iline) != after:
        print("err on", i//3*4-3)
'''