import sys

if (len(sys.argv) != 2):
    exit("sol.py <N>")
N = int(sys.argv[1])

e1 = 0
e2 = 1
L = [3,7]

while len(L) < N+10:
    s = L[e1] + L[e2]
    if s <= 9:
        L.append(s)
    else:
        L.append(1)
        L.append(s%10)

    e1 = (e1 + L[e1] + 1) % len(L)
    e2 = (e2 + L[e2] + 1) % len(L)

print("".join(map(str,L[N:N+10])))