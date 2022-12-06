import re
import sys
from dataclasses import dataclass

INITIAL_X = 0
INITIAL_Y = 0

DRAG = 1
GRAVITY = 1

@dataclass
class Trench:
    left: int
    right: int
    bottom: int
    top: int

class Probe:
    def __init__(self, vx, vy):
        self.x = INITIAL_X
        self.y = INITIAL_Y
        
        self.vx = vx
        self.vy = vy

    def move(self):
        self.x += self.vx
        self.y += self.vy

        self.vx = max(0, self.vx - DRAG)
        self.vy -= GRAVITY

    def is_in_trench(self, trench: Trench):
        return (trench.left <= self.x <= trench.right) and (trench.bottom <= self.y <= trench.top)
    
    def is_under_trench(self, trench: Trench):
        return self.y < trench.bottom

input_search = re.search(r"target area: x=(.*)\.\.(.*), y=(.*)\.\.(.*)", input())
trench = Trench(*map(int, input_search.groups()))

valid_velocities = 0
MAX_VY = int(sys.argv[1])
for vx in range(trench.right + 1):
   for vy in range(trench.bottom, MAX_VY + 1):
       probe = Probe(vx, vy)
       while not probe.is_under_trench(trench):
           if probe.is_in_trench(trench):
               valid_velocities += 1
               break
           probe.move()

print(valid_velocities)
