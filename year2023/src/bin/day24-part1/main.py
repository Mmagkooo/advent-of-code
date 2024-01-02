"""Solving a system of equations"""

from typing import Optional
import sys

import numpy as np


class Hailstone:
    """Abstraction of a hailstone particle"""

    p: np.array
    v: np.array

    def __init__(self, p, v):
        """Create new instance"""
        self.p = p
        self.v = v

    def __repr__(self):
        return f"Hailstone(v={self.v}, p={self.p})"

    def __str__(self):
        return repr(self)


def parse_vec(s: str) -> np.array:
    """Parse s to numpy array"""
    v = [int(w) for w in s.split(", ")]
    assert len(v) == 3
    return np.array(v[:2])


def parse_hailstone(s: str) -> Hailstone:
    """Parse s to Hailstone"""
    parts = [parse_vec(w) for w in s.split(" @ ")]
    assert len(parts) == 2
    return Hailstone(parts[0], parts[1])


def intersection_times(h_x: Hailstone, h_y: Hailstone) -> Optional[np.array]:
    """Intersect the two hailstones"""
    mat = np.array([[h_x.v[0], -h_y.v[0]], [h_x.v[1], -h_y.v[1]]])
    rhs = np.array([h_y.p[0] - h_x.p[0], h_y.p[1] - h_x.p[1]])
    try:
        return np.linalg.solve(mat, rhs)
    except np.linalg.LinAlgError:
        return None


MIN_P = np.array([200000000000000] * 2)
MAX_P = np.array([400000000000000] * 2)


def within_boundaries(p: np.array) -> bool:
    """Return True if within boundaries, otherwise False"""
    return (MIN_P <= p).all() and (p <= MAX_P).all()


def main():
    """Main method"""
    hailstones = [parse_hailstone(line.strip()) for line in sys.stdin]
    sol = 0
    for i, h_x in enumerate(hailstones):
        for j in range(i + 1, len(hailstones)):
            h_y = hailstones[j]
            t = intersection_times(h_x, h_y)
            if t is None:
                continue
            t_x, t_y = t
            if t_x <= 0 or t_y <= 0:
                continue
            intersection_p = h_x.p + h_x.v * t_x
            if within_boundaries(intersection_p):
                sol += 1

    print(sol)


if __name__ == "__main__":
    main()
