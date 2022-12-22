"""
Use this like:
python src/bin/day21-part1/main.py < data/day21/input.txt | python
"""

import sys

for line in sys.stdin:
    func_name, params = line.strip().split(": ")
    print(f"{func_name} = lambda: ", end="")

    params = params.split()
    if len(params) == 1:
        print(params[0])
    
    elif len(params) == 3:
        print(f"{params[0]}() {params[1]} {params[2]}()")
    
    else:
        raise ValueError("Unknown len:", len(params))

print("print(root())")
    