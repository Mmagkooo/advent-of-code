import numpy as np
from itertools import permutations

DIM = 3

transformations = []

def rek(t, perm, i, ret):
    if i == DIM:
        ret.append(t)
        return
    
    t1 = t.copy()
    t1[i][perm[i]] = 1
    rek(t1, perm, i+1, ret)

    t2 = t.copy()
    t2[i][perm[i]] = -1
    rek(t2, perm, i+1, ret)

for perm in permutations(range(DIM)):
    transformation = np.zeros([DIM, DIM])
    for row, col in enumerate(perm):
        transformation[row][col] = 1

    rek(transformation, perm, 0, transformations)
