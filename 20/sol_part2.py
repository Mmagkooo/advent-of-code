INF = int(1e9)

class piece:
    def __init__(self, x, y, d):
        self.x = x
        self.y = y
        self.d = d

moves = {
    'N': (0, -1),
    'W': (-1, 0),
    'S': (0, 1),
    'E': (1, 0)
}

def rek(sx, sy, i, sd):
    storage = []
    tmp_storage = [piece(sx, sy, sd)]
    while i < len(L) and L[i] != ')':
        curr = L[i]
        if curr == '|':
            for p in tmp_storage:
                storage.append(p)
            tmp_storage = [piece(sx, sy, sd)]
        elif curr == '(':
            for p in tmp_storage:
                new_ends, i = rek(p.x, p.y, i+1, p.d)
                storage.extend(new_ends)
        elif curr in moves:
            movex, movey = moves[curr]
            for p in tmp_storage:
                p.x += movex
                p.y += movey
                p.d += 1
                
                sofar = m.get((p.x, p.y), INF)
                m[p.x,p.y] = min(sofar, p.d)
        
        i += 1
        
    if i == len(L): return
    
    storage.extend(tmp_storage)

    return storage, i
        
    

L = input()[1:-1]
m = {(0,0): 0}
rek(0, 0, 0, 0)

print(m[max(m, key=lambda k: m[k])])
print(len(list(filter(lambda k: m[k] >= 1000, m))))

minx, miny = min(m.keys())
maxx, maxy = max(m.keys())
M = [[None for _ in range(maxy-miny+1)] for _ in range(maxx-minx+1)]

for x, y in m:
    M[x-minx][y-miny] = m[x,y]

for row in M:
    for el in row:
        print("{:2}".format(el), end=" ")
    print()