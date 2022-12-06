import sys

cnt = 0
for line in sys.stdin:
    output = line.strip().split(" | ")[1].split(" ")
    cnt += sum([len(o) in [2, 3, 4, 7] for o in output])

print(cnt)
