crabs = [int(i) for i in input().split(",")]

def sum_n(n):
    return n*(n+1) // 2

def calc_fuel(crabs, pos):
    return sum(map(lambda crab: sum_n(abs(crab-pos)), crabs))

mini, maxi = min(crabs), max(crabs)

min_pos = mini
min_val = calc_fuel(crabs, min_pos)
for pos in range(mini+1, maxi+1):
    val = calc_fuel(crabs, pos)
    if val < min_val:
        min_pos = pos
        min_val = val

print(min_val)
