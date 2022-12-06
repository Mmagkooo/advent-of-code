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
        
        self.hash = x
        self.hash = self.hash*33 + y
        self.hash = self.hash*33 + z
        self.hash = self.hash*33 + r
    
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
        if self == other:
            return True
        if type(self) != type(other):
            return False
        return self.x == other.x\
            and self.y == other.y\
            and self.z == other.z\
            and self.r == other.r
    
    def __hash__(self):
        return self.hash

    def __str__(self):
        return f"pos=<{self.x},{self.y},{self.z}>, r={self.r}"

class NanobotCollection:
    def __init__(self, nanobots):
        self.nanobots = set(nanobots)

    def intersects_with(self, other_nanobot):
        for my_nanobot in self.nanobots:
            if not my_nanobot.intersects_with(other_nanobot):
                return False
        return True

    def __len__(self):
        return len(self.nanobots)

    def add(self, nanobot):
        self.nanobots.add(nanobot)

    def __contains__(self, nanobot):
        return nanobot in self.nanobots
    
    def __iter__(self):
        return iter(self.nanobots)
    
    def __str__(self):
        return "{" + ", ".join(map(str, self.nanobots)) + "}"

class Point:
    def __init__(self, x, y, z):
        self.x = x
        self.y = y
        self.z = z
        
    def __str__(self):
        return f"Point({self.x}, {self.y}, {self.z})"

t1 = time.time()

#pos=<0,0,0>, r=4
LINE_RE = re.compile(r'^pos=<(.*),(.*),(.*)>, r=(.*)$')

nanobots = []
for line in sys.stdin:
    nanobot_params = map(int, re.findall(LINE_RE, line)[0])
    nanobot = Nanobot(*nanobot_params)
    nanobots.append(nanobot)

nanobot_collections = [NanobotCollection([n]) for n in nanobots]
for nanobot_collection in nanobot_collections:
    for nanobot in nanobots:
        if nanobot in nanobot_collection:
            continue
        if nanobot_collection.intersects_with(nanobot):
            nanobot_collection.add(nanobot)

max_collection = max(nanobot_collections, key=len)
print(len(max_collection))
"""
        Ovaj postupak pretpostavlja da je tražena točka jedan od vrhova manhattan sfera.
        To ne mora biti slučaj (npr. ako pričamo o 2D: sjecište dva pravokutnika; pravokutnik je u općem slučaju sjecište dva kvadrata)
"""
for n in max_collection:
    deltas = [-1, 0, 1]
    for dx, dy, dz in itertools.product(deltas, deltas, deltas):
        p = Point(n.x + dx*n.r, n.y + dy*n.r, n.z + dz*n.r)
        for other_n in max_collection:
            if p not in other_n:
                break
        else:
            print(p)

print("time passed: ", time.time() - t1)