import sys

def to_str(m):
    keys = sorted(m.keys())
    l = []
    for key in keys:
        l.append(m[key])
    return "".join(l)
    
s = input()
m = {}
leftmost = len(s)
rightmost = -1
for i, c in enumerate(s):
    m[i] = c
    if c == "#" and i < leftmost:
        leftmost = i
    elif c == "#" and i > rightmost:
        rightmost = i

for i in range(1, 5):
    m[leftmost-i] = m[rightmost+i] = '.'
        
rules = {}
for line in sys.stdin:
    pattern, res = line.strip().split(" => ")
    rules[pattern] = res

for iteration in range(200):
    nm = {}
    for i in range(leftmost-4, rightmost+4):
        pattern = ""
        for d in range(-2, 2+1):
            pattern += m.get(i+d, '.')
        nm[i] = rules[pattern]
    m = nm
    
    leftmost = 1e9
    rightmost = -1e9
    for i in m:
        if m[i] == '#':
            if i < leftmost:
                leftmost = i
            if i > rightmost:
                rightmost = i
                
    #for i in range(1, 5):
        #m[leftmost-i] = m[rightmost+i] = '.'
    print("i={}, left={}, right={}".format(iteration+1, leftmost, rightmost))
    print(to_str(m))
    print(sum([i for i in m if m[i] == '#']))
    print()