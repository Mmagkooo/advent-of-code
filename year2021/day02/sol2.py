import sys

horizontal = depth = aim = 0
for line in sys.stdin:
    command, amount = line.split()
    amount = int(amount)
    if command == "forward":
        horizontal += amount
        depth += aim * amount
    elif command == "up":
        aim -= amount
    else:
        aim += amount

print(horizontal*depth)

