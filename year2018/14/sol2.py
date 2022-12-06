import sys

if (len(sys.argv) != 2):
    exit("sol.py <N>")
N = list(map(int, sys.argv[1]))
Nlen = len(N)

e1 = 0
e2 = 1
L = [3,7]

while True:
    s = L[e1] + L[e2]
    if s <= 9:
        L.append(s)
        if L[-Nlen:] == N:
            break
    else:
        L.append(1)
        if L[-Nlen:] == N:
            break
        L.append(s%10)
        if L[-Nlen:] == N:
            break

    e1 = (e1 + L[e1] + 1) % len(L)
    e2 = (e2 + L[e2] + 1) % len(L)

print(len(L)-Nlen)