import sys

lines = [line.strip() for line in sys.stdin]

m = {}
X = 0
Y = 1

def sign(x):
    if x > 0:
        return 1
    if x == 0:
        return 0
    return -1

for line in lines:
    start_str, end_str = line.split(" -> ")
    start = tuple(int(val) for val in start_str.split(","))
    end = tuple(int(val) for val in end_str.split(","))

    step = sign(end[X] - start[X]), sign(end[Y] - start[Y])

    current = start[X], start[Y]
    if current not in m:
        m[current] = 0
    m[current] += 1

    while current != end:
        current = current[X] + step[X], current[Y] + step[Y]
        if current not in m:
            m[current] = 0
        m[current] += 1

cnt = 0
for key in m:
    if m[key] > 1:
        cnt += 1

print(cnt)
