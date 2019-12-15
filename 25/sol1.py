import sys

LIMIT = 3

def manhattan_distance(star1, star2):
    return sum(map(lambda pair: abs(pair[0]-pair[1]), zip(star1, star2)))

stars = []
with open(sys.argv[1]) as f:
    for line in f.readlines():
        star = list(map(int, line.strip().split(",")))
        stars.append(star)

parent = list(range(len(stars)))
def get_parent(i):
    if parent[i] == i:
        return i
    return get_parent(parent[i])

for i, star in enumerate(stars):
    for j, other_star in enumerate(stars):
        if manhattan_distance(star, other_star) <= LIMIT:
            pi = get_parent(i)
            pj = get_parent(j)
            parent[pi] = pj

distinct_parents = set()
for i, star in enumerate(stars):
    pi = get_parent(i)
    distinct_parents.add(pi)
print(len(distinct_parents))