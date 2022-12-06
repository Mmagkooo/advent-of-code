import re
import sys

block_pattern = r"""inp w
mul x 0
add x z
mod x 26
div z (\d*)
add x (-?\d*)
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y (-?\d*)
mul y x
add z y
"""

content = sys.stdin.read()

print("z0 = 0")
print("read_input = lambda: None")

i = 1
while True:
    matched = re.search(block_pattern, content)
    if not matched:
        break

    g = matched.groups()
    print(f"""
w{i} = read_input()
z{i} = z{i-1} // {g[0]}
if (z{i-1} % 26) + {g[1]} != w{i}:
    z{i} = 26 * z{i} + w{i} + {g[2]}
""", end="")
    content = content[matched.end():]
    i += 1
