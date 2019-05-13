import sys
from time import time
start_time = time()
ROCKY = 0
WET = 1
NARROW = 2

MOD = 20183
INF = int(1e9)

if len(sys.argv) != 4:
    print(__file__ + " <DEPTH> <TARGET_X> <TARGET_Y>")
    exit(1)

DEPTH, Tx, Ty = map(int, sys.argv[1:])

def rek(x, y, t, wt, torch, gear, prevx, prevy, ptorch, pgear):
    stack = [(x, y, t, wt, torch, gear, prevx, prevy, ptorch, pgear)]
    while stack:
    #def rek(x, y, t, wt, torch, gear, prevx, prevy, ptorch, pgear):
        x, y, t, wt, torch, gear, prevx, prevy, ptorch, pgear = stack.pop()
        if x < 0 or y < 0 or x >= len(field) or y >= len(field[0]):
            continue
        region = field[x][y]
        if region == ROCKY and not torch and not gear:
            continue
        if region == WET and torch:
            continue
        if region == NARROW and gear:
            continue
        if t+wt >= sum(rek.memo.get((x, y, torch, gear), (INF, INF))):
            continue
        
        rek.memo[x,y,torch,gear] = t, wt
        rek.prev[x,y,torch,gear] = prevx, prevy, ptorch, pgear
        
        stack.append((x-1, y, t+1, wt, torch, gear, x, y, torch, gear))
        stack.append((x-1, y, t+1, wt+7, not torch, gear, x, y, torch, gear))
        stack.append((x-1, y, t+1, wt+7, torch, not gear, x, y, torch, gear))
        stack.append((x-1, y, t+1, wt+7, not torch, not gear, x, y, torch, gear))
        
        stack.append((x+1, y, t+1, wt, torch, gear, x, y, torch, gear))
        stack.append((x+1, y, t+1, wt+7, not torch, gear, x, y, torch, gear))
        stack.append((x+1, y, t+1, wt+7, torch, not gear, x, y, torch, gear))
        stack.append((x+1, y, t+1, wt+7, not torch, not gear, x, y, torch, gear))
        
        stack.append((x, y-1, t+1, wt, torch, gear, x, y, torch, gear))
        stack.append((x, y-1, t+1, wt+7, not torch, gear, x, y, torch, gear))
        stack.append((x, y-1, t+1, wt+7, torch, not gear, x, y, torch, gear))
        stack.append((x, y-1, t+1, wt+7, not torch, not gear, x, y, torch, gear))
        
        stack.append((x, y+1, t+1, wt, torch, gear, x, y, torch, gear))
        stack.append((x, y+1, t+1, wt+7, not torch, gear, x, y, torch, gear))
        stack.append((x, y+1, t+1, wt+7, torch, not gear, x, y, torch, gear))
        stack.append((x, y+1, t+1, wt+7, not torch, not gear, x, y, torch, gear))
    
rek.memo = {}
rek.prev = {}

xlim = 2*Tx+1
ylim = 2*Ty+1

dp = [[1 for _ in range(ylim)] for _ in range(xlim)]
field = [[0 for _ in range(ylim)] for _ in range(xlim)]
for x in range(xlim):
    dp[x][0] = (16807*x + DEPTH) % MOD
    field[x][0] = dp[x][0] % 3
for y in range(ylim):
    dp[0][y] = (48271*y + DEPTH) % MOD
    field[0][y] = dp[0][y] % 3

for x in range(1, xlim):
    for y in range(1, ylim):
        dp[x][y] = (dp[x-1][y]*dp[x][y-1] + DEPTH) % MOD
        field[x][y] = dp[x][y] % 3

dp[0][0] = dp[Tx][Ty] = DEPTH % MOD
field[0][0] = field[Tx][Ty] = DEPTH % MOD % 3

rek(x=0, y=0, t=0, wt=0, torch=True, gear=False, prevx=None, prevy=None, ptorch=None, pgear=None)
print(rek.memo.get((Tx,Ty,True,False)), rek.memo.get((Tx,Ty,True,True)))

path = []
curr = Tx, Ty, True, False
while True:
    if curr[0] is None: break
    path.append(curr)
    curr = rek.prev[curr]
for el in path[::-1]:
    print(el)
#print(path[::-1])
#print(rek.memo.get((Tx,Ty,False,False)), rek.memo.get((Tx,Ty,True,False)), rek.memo.get((Tx,Ty,False,True)), rek.memo.get((Tx,Ty,True,True)))
print(time()-start_time)