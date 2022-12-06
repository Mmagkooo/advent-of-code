import sys

def bit_filter(lines, bit, comparator):
    cnt = [0, 0]
    for line in lines:
        cnt[int(line[bit])] += 1

    criterion = comparator(cnt)
    ret = []
    for line in lines:
        if int(line[bit]) == criterion:
            ret.append(line)
    return ret

def filter_loop(lines, comparator):
    bit = 0
    while len(lines) > 1:
        lines = bit_filter(lines, bit, comparator)
        bit += 1
    return int(lines[0], 2)

lines = [line.strip() for line in sys.stdin]
oxygen = filter_loop(lines, lambda cnt: 1 if cnt[1] >= cnt[0] else 0)
carbon = filter_loop(lines, lambda cnt: 0 if cnt[0] <= cnt[1] else 1)
print(oxygen, carbon, oxygen * carbon)
