import sys

lines = [line.strip() for line in sys.stdin]
score_table = {
    ")": 3,
    "]": 57,
    "}": 1197,
    ">": 25137
}

mapping = {
    "(": ")",
    "[": "]",
    "{": "}",
    "<": ">"
}

def parse(line):
    stack = []
    for char in line:
        if char in mapping:
            stack.append(char)
        elif not stack:
            return char
        elif mapping[stack[-1]] != char:
            return char
        else:
            stack.pop()
    return None

score = 0
for line in lines:
    illegal = parse(line)
    if illegal:
        score += score_table[illegal]

print(score)
