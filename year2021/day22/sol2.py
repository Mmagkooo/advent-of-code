"""Day 22"""

import sys
import re
from dataclasses import dataclass
import functools
from typing import List

from sortedcontainers import SortedList

LINE_REGEX = r"^(\w*) x=(.*)\.\.(.*),y=(.*)\.\.(.*),z=(.*)\.\.(.*)$"


@dataclass(frozen=True)
class Cuboid:
    """Cuboid abstraction"""

    x1: int
    x2: int
    y1: int
    y2: int
    z1: int
    z2: int
    on: bool
    time: int


def parse_state(state_str: str) -> bool:
    """Parse onness"""
    if state_str == "on":
        return True
    if state_str == "off":
        return False
    raise ValueError(f"Invalid state: {state_str}")


def read_cuboids() -> List[Cuboid]:
    """Read from stdin"""

    cuboids: List[Cuboid] = []
    for time, line in enumerate(l for l in sys.stdin if not l.startswith("#")):
        state, *numeric_input = re.search(LINE_REGEX, line.strip()).groups()
        cuboid = Cuboid(*map(int, numeric_input), on=parse_state(state), time=time)
        cuboids.append(cuboid)

    return cuboids


@dataclass(frozen=True)
@functools.total_ordering
class CuboidEvent:
    """Cuboid start or end"""

    dist: int
    start: bool
    """Start or end"""
    on: bool
    time: int

    def _to_comparable(self):
        return (self.dist, self.start, self.on, self.time)

    def __lt__(self, other: "CuboidEvent") -> bool:
        # return (self.dist, self.start) < (other.dist, other.start)
        if self.dist == other.dist:
            if self.start == other.start:
                return False
            return self.start is False  # ending event has precedence
        return self.dist < other.dist


def to_axis_events(
    cuboids: List[Cuboid], start_prop: str, end_prop: str
) -> List[CuboidEvent]:
    """Convert list of cuboids to list of cuboid events"""
    events = []
    for c in cuboids:
        events.append(
            CuboidEvent(dist=getattr(c, start_prop), start=True, on=c.on, time=c.time)
        )
        events.append(
            CuboidEvent(
                dist=getattr(c, end_prop) + 1, start=False, on=c.on, time=c.time
            )
        )

    return events


def main():
    """Main function"""

    cuboids = read_cuboids()

    on_count = 0

    x_events = sorted(to_axis_events(cuboids, start_prop="x1", end_prop="x2"))
    last_y_count = 0
    for x_i, x_event in enumerate(x_events):
        y_suitable_cuboids = [c for c in cuboids if c.x1 <= x_event.dist <= c.x2]
        y_count = 0
        y_events = sorted(
            to_axis_events(y_suitable_cuboids, start_prop="y1", end_prop="y2")
        )
        last_z_count = 0
        for y_i, y_event in enumerate(y_events):
            active_timestamps = SortedList()
            z_count = 0
            z_suitable_cuboids = [
                c for c in y_suitable_cuboids if c.y1 <= y_event.dist <= c.y2
            ]
            z_events = sorted(
                to_axis_events(z_suitable_cuboids, start_prop="z1", end_prop="z2")
            )
            for z_i, z_event in enumerate(z_events):
                if active_timestamps:
                    active_cuboid = cuboids[active_timestamps[-1]]
                    if active_cuboid.on:
                        z_count += z_event.dist - z_events[z_i - 1].dist

                if z_event.start:
                    active_timestamps.add(z_event.time)
                else:
                    active_timestamps.remove(z_event.time)

            y_count += (y_event.dist - y_events[y_i - 1].dist) * last_z_count
            last_z_count = z_count

        on_count += (x_event.dist - x_events[x_i - 1].dist) * last_y_count
        last_y_count = y_count

    print(on_count)


if __name__ == "__main__":
    main()
