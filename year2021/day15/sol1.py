import sys
import heapq

field = [[int(el) for el in line.strip()] for line in sys.stdin]

HEIGHT = len(field)
WIDTH = len(field[0])
INF = int(1e9)

remaining = []
for i in range(HEIGHT):
    for j in range(WIDTH):
        remaining.append((INF, (i, j)))

def pop_min(remaining, dists):
    while remaining:
        stored_dist, pos = heapq.heappop(remaining)
        if stored_dist == dists.get(pos, INF):
            return pos
    return None

def get_remaining_adj(pos):
    i, j = pos
    ret = []
    for di, dj in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
        ni, nj = i + di, j + dj
        if (ni < 0) or (ni >= HEIGHT):
            continue
        if  (nj < 0) or (nj >= WIDTH):
            continue
        ret.append((ni, nj))

    return ret

START_POS = 0, 0
dists = { START_POS: 0, (0, 1): field[0][1], (1, 0): field[1][0] }
first_dist, first_pos = heapq.heappop(remaining)
assert first_dist == INF and first_pos == START_POS
heapq.heappush(remaining, (field[0][1], (0, 1)))
heapq.heappush(remaining, (field[1][0], (1, 0)))

while remaining:
    min_pos = pop_min(remaining, dists)
    if min_pos is None:
        break
    min_pos_dist = dists[min_pos]

    for adj_pos in get_remaining_adj(min_pos):
        adj_i, adj_j = adj_pos
        old_dist = dists.get(adj_pos, INF)
        new_dist = min_pos_dist + field[adj_i][adj_j]
        if new_dist < old_dist:
            dists[adj_pos] = new_dist
            heapq.heappush(remaining, (new_dist, adj_pos))

print(dists[HEIGHT-1, WIDTH-1])
