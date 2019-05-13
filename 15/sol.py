from collections import deque
import sys

DMG = 3
INIT_HP = 200
INF = int(1e9)
orig = []
units = {}

def pretty():
    for i, row in enumerate(orig):
        for j, el in enumerate(row):
            print(el if (i,j) not in units else units[i,j][0], end="")
        print()

adj = [(-1,0), (0,-1), (0,+1), (+1,0)]
def bfs(srow, scol, grow, gcol, units):
    start = srow, scol
    goal = grow, gcol
    q = deque()
    q.append(((srow, scol), 0, None))
    seen = set()
    prev = {}
    while q:
        point, dist, prev_point = q.popleft()
        row, col = point
        
        if point == goal:
            point = prev_point
            while prev.get(point):
                if start == prev[point]:
                    break
                point = prev[point]
            return dist, point
        
        if row < 0 or col < 0 or row >= len(orig) or col >= len(orig[0]):
            continue
        if point in seen or orig[row][col] == '#' or (point in units and point != start):
            continue
          
        seen.add(point)
        prev[point] = prev_point
            
        for drow, dcol in adj:
            npair = nrow, ncol = row+drow, col+dcol
            q.append((npair, dist+1, point))

    return INF, None

cnt = {'E': 0, 'G': 0}
#f = open("sample1.txt")
#lines = f.readlines()
#f.close()
for i, line in enumerate(sys.stdin):
    orig.append([])
    for j, c in enumerate(line.strip()):
        if c == '#' or c == '.':
            orig[i].append(c)
        else:
            orig[i].append('.')
            cnt[c] += 1
            units[i,j] = c, INIT_HP

#pretty()
#while True:
#    row, col, grow, gcol = map(int,input().split())
#    print(bfs(row, col, grow, gcol, units))
            
units_list = sorted(units.keys())
    
round_i = 0
ok = True
while ok:
    #print(round_i)
    #pretty()
    #print([(key, units[key]) for key in sorted(units)])
    #print()
    
    deleted = set()
    
    for point in units_list:
        row, col = point
        if point in deleted:
            continue
        army, hp = units[point]
        
        if cnt['E'] == 0 or cnt['G'] == 0:
            ok = False
            break
        
        has_adj = False
        for drow, dcol in adj:
            npair = row+drow, col+dcol
            if npair in units and units[npair][0] != army:
                has_adj = True
                break
        
        if not has_adj:        
            min_dist = INF
            fpoint = None
            for npair in sorted(units):
                nrow, ncol = npair
                other_army, other_hp = units[npair]
                if other_army == army:
                    continue
                curr_dist, curr_fpoint = bfs(row, col, nrow, ncol, units)
                if curr_dist < min_dist:
                    min_dist = curr_dist
                    fpoint = curr_fpoint
                    
            if fpoint is None:
                continue # to the next point in units_list
            if fpoint in units:
                print(point, "ERR:", fpoint)
                exit()
            row, col = fpoint
            units[fpoint] = units[point]
            del units[point]

        potential_victims = []
        for drow, dcol in adj:
            npair = row+drow, col+dcol
            if npair in units:
                adj_army, adj_hp = units[npair]
                if adj_army != army:
                    potential_victims.append((npair, adj_army, adj_hp))
        
        if potential_victims:
            npair, adj_army, adj_hp = min(potential_victims, key=lambda victim: (victim[2], victim[0]))
            if adj_hp - DMG <= 0:
                del units[npair]
                deleted.add(npair)
                cnt[adj_army] -= 1
            else:
                units[npair] = adj_army, adj_hp - DMG
            #ok = True

    if ok:
        units_list = sorted(units.keys())
        round_i += 1
    
    
pretty()
print([(key, units[key]) for key in sorted(units)])
suma = sum(map(lambda unit: units[unit][1], units))
print("rounds", round_i)
print("suma", suma)
print(suma*round_i)