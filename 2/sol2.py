import fileinput

def diff(s1, s2):
    ret = 0
    for c1, c2 in zip(s1, s2):
        ret += c1 != c2
    return ret

L = [line.strip() for line in fileinput.input()]

for i, line in enumerate(L):
    for j in range(i+1, len(L)):
        if diff(line, L[j]) == 1:
            print(line, L[j], sep="\n")
    