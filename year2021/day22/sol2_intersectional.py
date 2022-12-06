import sys
import re
from dataclasses import dataclass
from typing import List

LINE_REGEX = r"^(\w*) x=(.*)\.\.(.*),y=(.*)\.\.(.*)$"

@dataclass
class Cuboid:
    x1: int
    x2: int
    y1: int
    y2: int
    state: str

    def intersect(self, other: "Cuboid"):
        """Not commutative."""

    def get_volume(self):
        return (self.x2 - self.x1 + 1) * (self.y2 - self.y1 + 1)

class Reactor:
    def __init__(self):
        self.cuboids: List[Cuboid] = []

    def add_cuboid(self, cuboid: Cuboid):
        pending_cuboids = [cuboid]
        while pending_cuboids:
            new_cuboids = []
            pending_cuboid = pending_cuboids.pop()
            for present_cuboid in self.cuboids:
                resulting_cuboids = present_cuboid.intersect(pending_cuboid)
                if len(resulting_cuboids) > 2:
                    pending_cuboids.extend(resulting_cuboids)
                else:
                    new_cuboids.append(pending_cuboid)
            self.cuboids = new_cuboids
                


reactor = Reactor()
for line in sys.stdin:
    state, *numeric_input = re.search(LINE_REGEX, line.strip())
    x1, x2, y1, y2 = map(int, numeric_input)
    cuboid = Cuboid(x1, x2, y1, y2, state)
    reactor.add_cuboid(cuboid)

sol = 0
for cuboid in reactor.cuboids:
    if cuboid.state == "on":
        sol += cuboid.get_volume()
print(sol)
