import sys

TIME = 10
MOD = int(1e13)

m = []

def pretty(m):
    for r in m:
        for el in r:
            print(el, end="")
        print()
    print()

def my_hash(m):
    h = 0
    for r in m:
        for el in r:
            h = (h*33 + ord(el)) % MOD
    return h

for line in sys.stdin:
    m.append(["-"] + list(line.strip()) + ["-"])

m.insert(0, ["-"]*len(m[0]))
m.append(["-"]*len(m[0]))

dis = [-1, -1, -1, 0, 0, 1, 1, 1]
djs = [-1, 0, 1, -1, 1, -1, 0, 1]

d = list(zip(dis, djs))

pretty(m)

memory = {}
memory[my_hash(m)] = (m, 0)

t = 1
while t < 441:
    nm = [['-']*len(m[0])]
    for i in range(1, len(m)-1):
        nr = []
        nr.append('-')
        for j in range(1, len(m[0])-1):
            adj = {'#': 0, '.': 0, '|': 0, '-': 0}
            for di, dj in d:
                adj[m[i+di][j+dj]] += 1
            curr = m[i][j]
            nr.append(curr)
            
            if curr == '.' and adj['|'] >= 3:
                nr[-1] = '|'    
            elif curr == '|' and adj['#'] >= 3:
                nr[-1] = '#'
            elif curr == '#' and (adj['#'] == 0 or adj['|'] == 0):
                nr[-1] = '.' 
        
        nr.append('-')
        nm.append(nr)
    nm.append(['-']*len(m[0]))
    
    m = nm

    h = my_hash(m)
    if h in memory:
        print(memory[h][1],t, memory[h][1]-t)
    else:
        memory[h] = (m, t)
    
    t += 1
    
pretty(m)

cnt = {'#': 0, '.': 0, '|': 0, '-': 0}
print(t)
for r in m:
    for el in r:
        cnt[el] += 1
print(cnt['|'], cnt['#'], cnt['|']*cnt['#'])