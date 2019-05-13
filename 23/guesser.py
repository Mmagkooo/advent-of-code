import sys
import re
import random
import copy

R = re.compile(r"^pos=<(-?\d*),(-?\d*),(-?\d*)>, r=(\d*)$")
INF = int(1e10)

class Point:
    def __init__(self, x, y, z):
        self.x = x
        self.y = y
        self.z = z

    def __repr__(self):
        return "Point({}, {}, {})".format(self.x, self.y, self.z)
    
    def __str__(self):
        return self.__repr__()
        
    def __eq__(self, other):
        return self.x == other.x and self.y == other.y and self.z == other.z
        
    @staticmethod
    def distance(p1, p2):
        return abs(p1.x-p2.x) + abs(p1.y-p2.y) + abs(p1.z-p2.z)

class Circle:
    def __init__(self, center, r):
        self.center = center
        self.r = r
    
    def __repr__(self):
        return "Circle({}, {})".format(self.center, self.r)
    
    def __str__(self):
        return self.__repr__()

def count(p):
    cnt = 0
    for circle in circles:
        cnt += Point.distance(p, circle.center) <= circle.r
    return cnt

def explore(p, n):
    p = Point(p.x, p.y, p.z)
    while True:
        oldp = Point(p.x, p.y, p.z)
        for attr in "xyz":
            delta = 1000
            while delta:
                old = getattr(p, attr)
                setattr(p, attr, old-delta)
                if count(p) != n:
                    setattr(p, attr, old)
                    delta //= 2
        if oldp == p:
            break
    return p

if len(sys.argv) != 2:
    print(__file__ + " <FILE>")
    exit()

circles = []

minx = miny = minz = INF
maxx = maxy = maxz = -INF

with open(sys.argv[1], "r") as f:
    for line in f.readlines():
        x, y, z, r = map(int, re.match(R, line).groups())
        circle = Circle(Point(x, y, z), r)
        circles.append(circle)
        minx = min(minx, x-r)
        miny = min(miny, y-r)
        minz = min(minz, z-r)
        
        maxx = max(maxx, x+r)
        maxy = max(maxy, y+r)
        maxz = max(maxz, z+r)

print("Loaded")

maxp = None
maxn = 0
while True:
    n_guess = int(input("N = "))
    old_maxn = maxn
    for _ in range(n_guess):
        rx = random.randint(minx, maxx)
        ry = random.randint(miny, maxy)
        rz = random.randint(minz, maxz)
        
        currp = Point(rx, ry, rz)
        currn = count(currp)
        if currn > maxn:
            maxn = currn
            maxp = currp
    
    if maxn > old_maxn:
        print("new random", maxp, maxn)
        maxp = explore(maxp, maxn)
        print("explored", maxp)
        