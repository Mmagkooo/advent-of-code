import sys

price = { "A": 1, "B": 10, "C": 100, "D": 1000 }

sol = 0
for line in sys.stdin:
    num, c = list(line.strip())
    sol += price[c.upper()] * int(num)
print(sol)
