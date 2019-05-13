import sys

INF = 1e9

def manhattan(p1, p2):
    return abs(p1[0]-p2[0]) + abs(p1[1]-p2[1])

L = []
minx, maxx, miny, maxy = INF, -INF, INF, -INF
p2i = {}
for i, line in enumerate(sys.stdin):
    x,y = map(int, line.split(", "))
    minx = min(minx, x)
    maxx = max(maxx, x)
    miny = min(miny, y)
    maxy = max(maxy, y)
    L.append((x, y))
    p2i[x,y] = i
    
mapa = {}
cnt = {}
out = set()
    
for x in range(minx-1, maxx+2):
    for y in range(miny-1, maxy+2):
        mini = [0]
        mindist = INF
        for i, p in enumerate(L):
            currdist = manhattan((x, y), p)
            if currdist < mindist:
                mindist = currdist
                mini = [i]
            elif currdist == mindist:
                mini.append(i)

        if len(mini) == 1:
            theind = mini[0]
            mapa[x,y] = theind
            if x == minx-1 or x == maxx+1 or y == miny-1 or y == maxy+1:
                out.add(theind)
        else:
            mapa[x,y] = -1
            
for y in range(miny-1, maxy+2):
    for x in range(minx-1, maxx+2):
        curri = mapa[x,y]
        print("{:2}".format(curri if curri != -1 else "."), end="")
        if curri != -1 and curri not in out:
            cnt[curri] = cnt.get(curri, 0)+1
    print()
maxi = max(cnt, key=lambda i: cnt[i])
print(maxi, cnt[maxi])
#print("DEBUG")
#for i in cnt:
#    print(i, cnt[i])