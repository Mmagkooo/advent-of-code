NEW_AGE = 8
OLD_AGE = 6

age2count = {age: 0 for age in range(NEW_AGE + 1)}
for age in map(int, input().split(",")):
    age2count[age] += 1

for day in range(256):
    old_zero_count = age2count[0]
    for age in range(0, NEW_AGE):
        age2count[age] = age2count[age + 1]
    age2count[OLD_AGE] += old_zero_count
    age2count[NEW_AGE] = old_zero_count

print(sum(age2count.values()))
