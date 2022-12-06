import sys
from copy import deepcopy

field = [[int(el) for el in line.strip()] for line in sys.stdin]

def join_horizontal(m1, m2):
    assert len(m1) == len(m2)

    joint = []
    for i in range(len(m1)):
        joint.append(m1[i] + m2[i])
    return joint

def join_vertical(m1, m2):
    joint = deepcopy(m1)
    for row2 in m2:
        joint.append(deepcopy(row2))
    return joint

def inc(m):
    ret = deepcopy(m)
    for i in range(len(ret)):
        for j in range(len(ret[i])):
            val = ret[i][j]
            if val == 9:
                ret[i][j] = 1
            else:
                ret[i][j] += 1
    return ret

joint_field = [[] for _ in range(len(field))]
joint_field = join_horizontal(joint_field, field)
for _ in range(4):
    field = inc(field)
    joint_field = join_horizontal(joint_field, field)

final_field = []
final_field = join_vertical(final_field, joint_field)
for _ in range(4):
    joint_field = inc(joint_field)
    final_field = join_vertical(final_field, joint_field)

for row in final_field:
    print("".join(map(str,row)))
