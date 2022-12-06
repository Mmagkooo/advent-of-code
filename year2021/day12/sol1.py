import sys

edges = {}
for line in sys.stdin:
    v1, v2 = line.strip().split("-")
    if v1 not in edges:
        edges[v1] = []
    edges[v1].append(v2)

    if v2 not in edges:
        edges[v2] = []
    edges[v2].append(v1)

memo = {}
def walk(v: str, path: tuple):
    if v == "end":
        return 1

    if v in path and v.islower():
        return 0

    path = path + (v,)
    if path in memo:
        return memo[path]

    total = 0
    for other_v in edges[v]:
        total += walk(other_v, path)

    memo[path] = total
    return total

print(walk("start", ()))
