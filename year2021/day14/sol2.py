import sys
from collections import defaultdict
ddint = lambda: defaultdict(int)

def get_pairs(seq):
    pairs = ddint()
    for i in range(len(seq) - 1):
        pairs[seq[i] + seq[i+1]] += 1
    return pairs

def replace(pairs, rules):
    new_pairs = ddint()
    for pair in pairs:
        pair_amount = pairs[pair]
        new_val = rules[pair]
        new_pair1 = pair[0] + new_val
        new_pairs[new_pair1] += pair_amount

        new_pair2 = new_val + pair[1]
        new_pairs[new_pair2] += pair_amount

    return new_pairs

def get_freq(pairs, sequence):
    freq = ddint()
    for pair in pairs:
        pair_amount = pairs[pair]
        freq[pair[0]] += pair_amount
        freq[pair[1]] += pair_amount

    for letter in freq:
        freq[letter] //= 2
    
    assert len(sequence) >= 2
    freq[sequence[0]] += 1
    freq[sequence[-1]] += 1
    return freq

lines = [line.strip() for line in sys.stdin]
sequence = list(lines[0])
rules = dict([line.split(" -> ") for line in lines[2:]])
pairs = get_pairs(sequence)

N_STEPS = 40
for step_i in range(N_STEPS):
    pairs = replace(pairs, rules)

freq = get_freq(pairs, sequence)
freq_values = freq.values()
max_freq = max(freq_values)
min_freq = min(freq_values)
print(max_freq - min_freq)