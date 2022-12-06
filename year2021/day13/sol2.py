import sys

lines = [line.strip() for line in sys.stdin]
empty_i = lines.index("")

max_x = max_y = 0
paper = set()
for line in lines[:empty_i]:
    x, y = map(int, line.split(","))
    paper.add((x, y))

def print_paper(paper):
    max_x = max_y = 0
    for x, y in paper:
        max_x = max(max_x, x)
        max_y = max(max_y, y)

    for y in range(max_y + 1):
        for x in range(max_x + 1):
            print("#" if (x, y) in paper else ".", end="")
        print()

def fold(paper, axis, fold_pos):
    max_x = max_y = 0
    for x, y in paper:
        max_x = max(max_x, x)
        max_y = max(max_y, y)

    new_paper = set()
    if axis == "x":
        new_max_x = max(fold_pos, max_x - fold_pos) - 1
        for y in range(max_y + 1):
            for x in range(new_max_x + 1):
                candidate1 = (fold_pos - x - 1, y)
                candidate2 = (fold_pos + x + 1, y)
                if candidate1 in paper or candidate2 in paper:
                    new_paper.add((x, y))

    if axis == "y":
        new_max_y = max(fold_pos, max_y - fold_pos) - 1
        for x in range(max_x + 1):
            for y in range(new_max_y + 1):
                candidate1 = (x, fold_pos - y - 1)
                candidate2 = (x, fold_pos + y + 1)
                if candidate1 in paper or candidate2 in paper:
                    new_paper.add((x, y))

    return new_paper

for line in lines[empty_i + 1:]:
    axis_raw, pos_raw = line.split("=")
    axis = axis_raw[-1]
    pos = int(pos_raw)
    paper = fold(paper, axis, pos)

print_paper(paper)
