import sys

lines = [line.strip() for line in sys.stdin]

sequence = list(lines[0])
rules = dict([line.split(" -> ") for line in lines[2:]])

def replace(seq, rules):
    seq_len = len(seq)
    new_seq = [seq[0]]
    for i in range(seq_len - 1):
        pattern = seq[i] + seq[i+1]
        new_seq.append(rules[pattern])
        new_seq.append(seq[i+1])

    return new_seq

N_STEPS = 10
for _ in range(N_STEPS):
    sequence = replace(sequence, rules)

freq = {}
for c in sequence:
    if c not in freq:
        freq[c] = 0
    freq[c] += 1

values = freq.values()
max_val = max(values)
min_val = min(values)
print(max_val - min_val)
