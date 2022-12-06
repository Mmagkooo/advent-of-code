import sys

lines = [line.strip() for line in sys.stdin]
score_table = {
    ")": 1,
    "]": 2,
    "}": 3,
    ">": 4
}

mapping = {
    "(": ")",
    "[": "]",
    "{": "}",
    "<": ">"
}

class ParseError(ValueError):
    pass

def parse(line):
    stack = []
    for char in line:
        if char in mapping:
            stack.append(char)
        elif not stack:
            raise ParseError
        elif mapping[stack[-1]] != char:
            raise ParseError
        else:
            stack.pop()
    return stack

def calc_score(stack):
    score = 0
    while stack:
        char = stack.pop()
        score = 5 * score + score_table[mapping[char]]
    return score

scores = []
for line in lines:
    try:
        remaining = parse(line)
    except ParseError:
        continue

    score = calc_score(remaining)
    scores.append(score)

print(sorted(scores)[len(scores) // 2])
