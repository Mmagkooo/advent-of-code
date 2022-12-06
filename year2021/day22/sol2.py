import sys
import re
from dataclasses import dataclass
from typing import List

LINE_REGEX = r"^(\w*) x=(.*)\.\.(.*),y=(.*)\.\.(.*)$"

@dataclass(frozen=True)
class Cuboid:
    x1: int
    x2: int
    y1: int
    y2: int
    state: str
    time: int

class LightTracker:
    def __init__(self):
        self.offs = set()
        self.ons = set()

    def track_event(self, is_start: bool, cuboid: Cuboid):
        if cuboid.state == "on":
            if is_start:
                self.ons.add(cuboid.time)
            else:
                self.ons.remove(cuboid.time)
        elif cuboid.state == "off":
            if is_start:
                self.offs.add(cuboid.time)
            else:
                self.offs.remove(cuboid.time)
        else:
            raise ValueError

    def currently_on(self):
        last_on = -1 if not self.ons else max(self.ons)
        last_off = -1 if not self.offs else max(self.offs)
        if last_on == last_off:
            if last_on == -1:
                return False
            raise ValueError
        return last_on > last_off

cuboids: List[Cuboid] = []
for time, line in enumerate(sys.stdin):
    if line.startswith("#"):
        continue
    state, *numeric_input = re.search(LINE_REGEX, line.strip()).groups()
    x1, x2, y1, y2 = map(int, numeric_input)
    cuboid = Cuboid(x1, x2, y1, y2, state, time)
    cuboids.append(cuboid)

def create_events(cuboids):
    events = []
    for cuboid in cuboids:
        events.append((True, cuboid))
        events.append((False, cuboid))
    return events

def get_cuboids(events, predicate):
    cuboids = set()
    for _, cuboid in events:
        if predicate(cuboid):
            cuboids.add(cuboid)
    return cuboids

x_events = create_events(cuboids)
print("DEBUG x_events", x_events)
area = 0
last_x = None
for is_start, x_cuboid in sorted(x_events, key=lambda e: (e[1].x1 if e[0] else e[1].x2, e[0])): # sort using tuple to give precedence to closing events
    print("DEBUG is_start, x_cuboid", is_start, x_cuboid)
    x = x_cuboid.x1 if is_start else x_cuboid.x2
    y_events = create_events(get_cuboids(x_events, lambda c: c.x1 <= x <= c.x2))
    # TODO create counter for how many new cuboids at e.g. x==2 (solves case of multiple starting/ending on same line)
    last_y = None
    y_length = 0
    # TODO does x axis have to take care of whether it currently is in light or not?
    light_tracker = LightTracker()
    for is_start, y_cuboid in sorted(y_events, key=lambda e: (e[1].y1 if e[0] else e[1].y2, e[0])):
        y = y_cuboid.y1 if is_start else y_cuboid.y2
        if last_y is not None and light_tracker.currently_on():
            y_length += y - last_y + 1
            print("DEBUG y_length", y_length)
        else:
            y_length = 0

        light_tracker.track_event(is_start, y_cuboid)
        last_y = y

    if last_x is not None:
        x_diff = x - last_x + 1
        print("DEBUG x_diff", x_diff)
        area += y_length * x_diff
    last_x = x

print(area)
