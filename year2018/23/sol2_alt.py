import time
import sys
import re
import itertools

class Nanobot:
    def __init__(self, x, y, z, r):
        self.x = x
        self.y = y
        self.z = z
        self.r = r
        
        self._hash = x
        self._hash = self._hash*33 + y
        self._hash = self._hash*33 + z
        self._hash = self._hash*33 + r
    
    def intersects_with(self, other):
        dist = abs(self.x - other.x)\
            + abs(self.y - other.y)\
            + abs(self.z - other.z)
        return dist <= self.r + other.r
    
    def __contains__(self, point):
        dist = abs(self.x - point.x)\
            + abs(self.y - point.y)\
            + abs(self.z - point.z)
        return dist <= self.r
    
    def __eq__(self, other):
        if self is other:
            return True
        if type(self) != type(other):
            return False
        return self.x == other.x\
            and self.y == other.y\
            and self.z == other.z\
            and self.r == other.r
    
    def __hash__(self):
        return self._hash

    def __str__(self):
        return f"pos=<{self.x},{self.y},{self.z}>, r={self.r}"
    
    def __repr__(self):
        return self.__str__()


class Point:
    def __init__(self, x, y, z):
        self.x = x
        self.y = y
        self.z = z
        
    def __str__(self):
        return f"Point({self.x}, {self.y}, {self.z})"
    
    def __repr__(self):
        return self.__str__()

def count_contained_in(point, nanobots):
    cnt = 0
    for other_n in nanobots:
        if point in other_n:
            cnt += 1
    return cnt

t1 = time.time()

#pos=<0,0,0>, r=4
LINE_RE = re.compile(r'^pos=<(.*),(.*),(.*)>, r=(.*)$')

nanobots = []
with open(sys.argv[1]) as f:
    lines = f.readlines()
for line in lines:
    nanobot_params = map(int, re.findall(LINE_RE, line)[0])
    nanobot = Nanobot(*nanobot_params)
    nanobots.append(nanobot)

max_cnt = 0
max_points = []
for n in nanobots:
    points = [
        Point(n.x + n.r, n.y, n.z),
        Point(n.x - n.r, n.y, n.z),
        Point(n.x, n.y + n.r, n.z),
        Point(n.x, n.y - n.r, n.z),
        Point(n.x, n.y, n.z + n.r),
        Point(n.x, n.y, n.z - n.r),
    ]
    
    for p in points:
        cnt = count_contained_in(p, nanobots)
        if cnt > max_cnt:
            max_cnt = cnt
            max_points = [p]
        elif cnt == max_cnt:
            max_points.append(p)

print(max_cnt, max_points)


print("time passed: ", time.time() - t1)