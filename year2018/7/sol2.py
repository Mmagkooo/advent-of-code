import sys
from queue import PriorityQueue as PQ

def val(c):
    return ord(c)-ord('A')+1

N_WORKERS = 2
JOB_TIME = 0

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

workers = [0]*N_WORKERS
workersj = [None]*N_WORKERS
time = 0
while not q.empty() or workers != [0]*N_WORKERS:
    print(workers)
    print(workersj)
    print()
    time += 1
    for i in range(N_WORKERS):
        if workers[i] == 0:
            continue
        if workers[i] == 1:
            curr = workersj[i]
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
            workersj[i] = None
        workers[i] -= 1
        
    while 0 in workers and not q.empty():
        curr = q.get(False)
        if curr in seen: continue
        zero_index = workers.index(0)
        workers[zero_index] = JOB_TIME + val(curr)
        workersj[zero_index] = curr
        
print(time)