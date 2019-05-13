import sys
sys.path.append("../16")
import operators

instructions = [line.strip().split() for line in sys.stdin]

for ins in instructions:
    for j in range(1, len(ins)):
        ins[j] = int(ins[j])

preprocess, instructions = instructions[0], instructions[1:]
        
N = len(instructions)
ip = 0
ri = preprocess[1]
reg = [0, 0, 0, 0, 0, 0]

while True:
    if ip >= N: break
    ins = instructions[ip]
    op = ins[0]
    arg = ins[1:]
    reg[ri] = ip
    reg = list(getattr(operators, op)(reg, arg))
    ip = reg[ri] + 1