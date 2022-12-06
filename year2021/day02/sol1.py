import sys

horizontal = depth = 0
for line in sys.stdin:
    command, amount = line.split()
    amount = int(amount)
    if command == "forward":
        horizontal += amount
    elif command == "up":
        depth -= amount
    else:
        depth += amount

print(horizontal*depth)

