import sys

depths = [int(line) for line in sys.stdin]
cnt = 0
for i in range(1, len(depths)):
    if depths[i] > depths[i-1]:
        cnt += 1

print(cnt)
