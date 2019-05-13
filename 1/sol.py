import fileinput

s = 0
ss = set()
ss.add(s)
lines = []
for line in fileinput.input():
    lines.append(line)

i = 0
while True:
    s += int(lines[i])
    if s in ss:
        print(s)
        break
    ss.add(s)
    
    i = (i+1) % len(lines)
