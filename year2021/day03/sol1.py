import sys

lines = [line.strip() for line in sys.stdin]
line_size = len(lines[0])
cnt = [0] * line_size
for line in lines:
    for i, val in enumerate(line):
        val = int(val)
        cnt[i] += val

gamma = 0
epsilon = 0
for val in cnt:
    if val > len(lines) // 2:
        gamma = 2*gamma + 1
        epsilon = 2*epsilon
    else:
        gamma = 2*gamma
        epsilon = 2*epsilon + 1

print(gamma, epsilon, gamma*epsilon)
