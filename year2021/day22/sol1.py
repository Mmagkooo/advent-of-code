import sys
import re

LINE_REGEX = r"^(\w*) x=(.*)\.\.(.*),y=(.*)\.\.(.*),z=(.*)\.\.(.*)$"
BORDER = 50

reactor = {}
for line in sys.stdin:
    line = line.strip()
    command, *numeric_input = re.search(LINE_REGEX, line).groups()
    x1, x2, y1, y2, z1, z2 = map(int, numeric_input)
    
    x_min = max(x1, -BORDER)
    x_max = min(x2, BORDER)
    for x in range(x_min, x_max + 1):
        y_min = max(y1, -BORDER)
        y_max = min(y2, BORDER)
        for y in range(y_min, y_max + 1):
            z_min = max(z1, -BORDER)
            z_max = min(z2, BORDER)
            for z in range(z_min, z_max + 1):
                reactor[x, y, z] = command

sol = 0
for x in range(-BORDER, BORDER + 1):
    for y in range(-BORDER, BORDER + 1):
        for z in range(-BORDER, BORDER + 1):
            sol += int(reactor.get((x, y, z), "off") == "on")
print(sol)
