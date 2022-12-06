import sys

ROCKY = 0
WET = 1
NARROW = 1

MOD = 20183

if len(sys.argv) != 4:
    print(__file__ + " <DEPTH> <TARGET_X> <TARGET_Y>")
    exit(1)

DEPTH, Tx, Ty = map(int, sys.argv[1:])

def pow(a, b):
    if b == 0: return 1
    ret = pow(a, b//2)
    tmp = a if b%2 else 1
    return (tmp*ret*ret) % MOD
    
dp = [[(0,0) for _ in range(Tx+1)] for _ in range(Ty+1)]
for x in range(Tx+1):
    dp[0][x] = (x, 0)
for y in range(Ty+1):
    dp[y][0] = (0, y)

for y in range(1, Ty+1):
    for x in range(1, Tx+1):
        upper_a, upper_b = dp[y-1][x]
        left_a, left_b = dp[y][x-1]
        dp[y][x] = (upper_a+left_a, upper_b+left_b)
    
#for y in range(Ty+1):
#    for x in range(Tx+1):
#        print(dp[y][x], end=" ")
#    print()
    
def erosion_level(x, y):
    a, b = dp[y][x]
    if (x == 0 and y == 0) or (x == Tx and y == Ty):
        p, q = 0, 0
    else:
        p, q = pow(48271, a), pow(16807, b)
    return (p*q + DEPTH) % MOD

sol = 0
for y in range(Ty+1):
    for x in range(Tx+1):
        #a, b = dp[y][x]
        #print(pow(48271, a)*pow(16807, b), end=" ")
        print(erosion_level(x, y), end=" ")
        sol += erosion_level(x, y)
    print()
print(sol)