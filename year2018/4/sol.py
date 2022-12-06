import sys
import re

L = sorted([line for line in sys.stdin])

mapa = {}

for line in L:
    h, m, a, ID = re.match(r'^\[\d{4}-\d{2}-\d{2} (\d{2}):(\d{2})\] (\w)\D*(\d*).*$', line).groups()
    m = int(m)
    if a == 'f':
        last = m
    elif a == 'G':
        elf = int(ID)
    elif a == 'w':
        if elf not in mapa:
            mapa[elf] = 60*[0]
        
        lista = mapa[elf]
        for minuta in range(last, m):
            lista[minuta] += 1
    
maxelf = 0
maxval = 0
for elf in mapa:
    currval = sum(mapa[elf])
    if currval > maxval:
        maxelf = elf
        maxval = currval
    
maxlist = mapa[maxelf]
maxind = max(range(60), key=lambda m: maxlist[m])
print(maxelf*maxind)