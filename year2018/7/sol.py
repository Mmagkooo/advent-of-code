import sys
from queue import PriorityQueue as PQ

m = {}
parents = {}

nodes = set()
has_parent = set()

for line in sys.stdin:
    x = line[5]
    y = line[36]
    
    nodes.add(x)
    has_parent.add(y)
    
    if x not in m:
        m[x] = []
    m[x].append(y)
    
    if y not in parents:
        parents[y] = []
    parents[y].append(x)
    
no_parent = nodes - has_parent

seen = set()
q = PQ()
for node in no_parent:
    q.put(node)

while not q.empty():
    curr = q.get(False)
    if curr in seen: continue
    seen.add(curr)
    if curr in m:
        for node in m[curr]:
            ok = True
            for parent in parents[node]:
                if parent not in seen:
                    ok = False
                    break
            if ok:
                q.put(node)
    print(curr, end="")