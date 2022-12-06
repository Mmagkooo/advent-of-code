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

def rek(x, y, t, torch, gear):
    stack = [(x, y, t, torch, gear)]
    while stack:
        x, y, t, torch, gear = stack.pop()
        if x < 0 or y < 0 or x >= len(field) or y >= len(field[0]):
            continue
        region = field[x][y]
        if region == ROCKY and not torch and not gear:
            continue
        if region == WET and torch:
            continue
        if region == NARROW and gear:
            continue
        if t >= rek.memo.get((x, y, torch, gear), INF):
            continue
        
        rek.memo[x,y,torch,gear] = t
        
        stack.append((x-1, y, t+1, torch, gear))
        stack.append((x-1, y, t+8, not torch, gear))
        stack.append((x-1, y, t+8, torch, not gear))
        stack.append((x-1, y, t+8, not torch, not gear))
        
        stack.append((x+1, y, t+1, torch, gear))
        stack.append((x+1, y, t+8, not torch, gear))
        stack.append((x+1, y, t+8, torch, not gear))
        stack.append((x+1, y, t+8, not torch, not gear))
        
        stack.append((x, y-1, t+1, torch, gear))
        stack.append((x, y-1, t+8, not torch, gear))
        stack.append((x, y-1, t+8, torch, not gear))
        stack.append((x, y-1, t+8, not torch, not gear))
        
        stack.append((x, y+1, t+1, torch, gear))
        stack.append((x, y+1, t+8, not torch, gear))
        stack.append((x, y+1, t+8, torch, not gear))
        stack.append((x, y+1, t+8, not torch, not gear))
    
rek.memo = {}

xlim = Tx+1+5
ylim = Ty+1+5

dp = [[0 for _ in range(ylim)] for _ in range(xlim)]
field = [[0 for _ in range(ylim)] for _ in range(xlim)]
for x in range(xlim):
    dp[x][0] = (16807*x + DEPTH) % MOD
    field[x][0] = dp[x][0] % 3
for y in range(ylim):
    dp[0][y] = (48271*y + DEPTH) % MOD
    field[0][y] = dp[0][y] % 3

for x in range(1, xlim):
    for y in range(1, ylim):
        if x == Tx and y == Ty:
            dp[x][y] = DEPTH % MOD
            field[x][y] = DEPTH % MOD % 3
        else:
            dp[x][y] = (dp[x-1][y]*dp[x][y-1] + DEPTH) % MOD
            field[x][y] = dp[x][y] % 3

dp[0][0] = dp[Tx][Ty] = DEPTH % MOD
field[0][0] = field[Tx][Ty] = DEPTH % MOD % 3

'''
print_map = {ROCKY: '.', WET: '=', NARROW: '|'}
for y in range(ylim):
    for x in range(xlim):
        if x == 0 and y == 0:
            print("M", end="")
        elif x == Tx and y == Ty:
            print("T", end="")
        else:
            print(print_map[field[x][y]], end="")
    print()
'''

rek(x=0, y=0, t=0, torch=True, gear=False)
print(rek.memo.get((Tx,Ty,True,False)), rek.memo.get((Tx,Ty,True,True)))

print(time()-start_time)