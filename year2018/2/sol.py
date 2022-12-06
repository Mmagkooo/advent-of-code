import fileinput

cnt2, cnt3 = 0, 0
for line in fileinput.input():
    m = {}
    for c in line:
        m[c] = m.get(c, 0) + 1
    for c in m:
        if m[c] == 2:
            cnt2 += 1
            break
    for c in m:
        if m[c] == 3:
            cnt3 += 1
            break
            
print(cnt2*cnt3)