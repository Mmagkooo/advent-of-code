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

sol = 0
dp = [[1 for _ in range(Ty+1)] for _ in range(Tx+1)]
for x in range(Tx+1):
    dp[x][0] = (16807*x + DEPTH) % MOD
    sol += dp[x][0] % 3
for y in range(Ty+1):
    dp[0][y] = (48271*y + DEPTH) % MOD
    sol += dp[0][y] % 3

for x in range(1, Tx+1):
    for y in range(1, Ty+1):
        dp[x][y] = (dp[x-1][y]*dp[x][y-1] + DEPTH) % MOD
        sol += dp[x][y] % 3

#dp[0][0] = dp[Tx][Ty] = 0
sol -= (dp[0][0] % 3) + (dp[Tx][Ty] % 3)
print(sol)
