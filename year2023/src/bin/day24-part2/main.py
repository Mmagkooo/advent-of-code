"""Solving a system of equations"""

import functools
import itertools
import sys
from typing import Optional
from multiprocessing import Process

import numpy as np
from numpy.typing import NDArray


class Hailstone:
    """Abstraction of a hailstone particle"""

    p: NDArray
    v: NDArray

    def __init__(self, p: NDArray, v: NDArray):
        """Create new instance"""
        self.p = p
        self.v = v

    def __repr__(self):
        return f"Hailstone(v={self.v}, p={self.p})"

    def __str__(self):
        return repr(self)


def parse_vec(s: str) -> NDArray:
    """Parse s to numpy array"""
    v = [int(w) for w in s.split(", ")]
    assert len(v) == 3
    return np.array(v)


def parse_hailstone(s: str) -> Hailstone:
    """Parse s to Hailstone"""
    p, v = [parse_vec(w) for w in s.split(" @ ")]
    return Hailstone(p, v)


def find_p(h_i: Hailstone, h_j: Hailstone, v: NDArray) -> Optional[NDArray]:
    """
    p + v*t_i = p_i + v_i*t_i
    p + v*t_j = p_j + v_j*t_i

    p_x + v_x*t_i = p_xi + v_xi*t_i => 1*p_x + 0*p_y + 0*p_z + (v_x - v_xi)*ti + 0*t_j = p_xi
    p_y + v_y*t_i = p_yi + v_yi*t_i => 0*p_x + 1*p_y + 0*p_z + (v_y - v_yi)*ti + 0*t_j = p_yi
    p_z + v_z*t_i = p_zi + v_zi*t_i => 0*p_x + 0*p_y + 1*p_z + (v_z - v_zi)*ti + 0*t_j = p_zi

    p_x + v_x*t_j = p_xj + v_xj*t_j => 1*p_x + 0*p_y + 0*p_z + 0*ti + (v_x - v_xj)*t_j = p_xj
    p_y + v_y*t_j = p_yj + v_yj*t_j => 0*p_x + 1*p_y + 0*p_z + 0*ti + (v_y - v_yj)*t_j = p_yj
    p_z + v_z*t_j = p_zj + v_zj*t_j => 0*p_x + 0*p_y + 1*p_z + 0*ti + (v_z - v_zj)*t_j = p_zj
    """

    coeff_matrix = np.array(
        [
            [1, 0, 0, v[0] - h_i.v[0], 0],
            [0, 1, 0, v[1] - h_i.v[1], 0],
            [0, 0, 1, v[2] - h_i.v[2], 0],
            [1, 0, 0, 0, v[0] - h_j.v[0]],
            [0, 1, 0, 0, v[1] - h_j.v[1]],
            # [0, 0, 1, 0, v[2] - h_j.v[2]],
        ]
    )
    b = np.array([h_i.p[0], h_i.p[1], h_i.p[2], h_j.p[0], h_j.p[1]])

    try:
        sol = np.linalg.solve(coeff_matrix, b)
    except np.linalg.LinAlgError:
        return None

    p, t = sol[:3], sol[3:]
    if (p > -EPS).all() and (t > -EPS).all():
        return p
    return None


EPS = 1e-9


def collides(stone: Hailstone, p_mat: NDArray, v_mat: NDArray) -> bool:
    """Check if stone collides with all hailstones"""
    t_mat = (p_mat - stone.p) / (stone.v - v_mat)
    t_equality = np.abs(t_mat - t_mat[:, 0].reshape(-1, 1)) < EPS
    return np.logical_or(t_equality, np.isnan(t_mat)).all()


def search(hailstones, v_ranges):
    """Try possible values for v"""
    p_mat = [h.p for h in hailstones]
    v_mat = [h.v for h in hailstones]
    for v in itertools.product(*v_ranges):
        v = np.array(v)
        h_i, h_j = hailstones[0], hailstones[1]

        p = find_p(h_i, h_j, v)
        if p is None or (np.abs(np.trunc(p) - p) > EPS).any():
            continue

        stone = Hailstone(p=p, v=v)

        if collides(stone, p_mat, v_mat):
            print(f"p={p.astype(str)}, v={v.astype(str)} => {sum(p)}")
            break

    else:
        # print("No suitable position")
        pass


def main():
    """Main method spawning multiple processes"""
    hailstones = [parse_hailstone(line.strip()) for line in sys.stdin]

    # finds solution if max_v = 250
    max_v = int(sys.argv[1])

    # search(hailstones, [range(-max_v, max_v + 1)] * 3)
    batch_size = max_v
    processes = []
    x_batch_size = batch_size // 2
    for i in range(-max_v, max_v, x_batch_size):
        for j in range(-max_v, max_v, batch_size):
            for k in range(-max_v, max_v, batch_size):
                p = Process(
                    target=functools.partial(search, hailstones),
                    args=[
                        [
                            range(i, i + x_batch_size),
                            range(j, j + batch_size),
                            range(k, k + batch_size),
                        ]
                    ],
                )
                p.start()
                print(f"Spawned processes for {(i, j, k)}")
                processes.append(p)

    # 16 - same as cores on my CPU
    print(f"Spawned {len(processes)} processes")
    for p in processes:
        p.join()


if __name__ == "__main__":
    main()
