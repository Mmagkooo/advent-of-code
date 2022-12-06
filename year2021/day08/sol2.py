import sys

sol = 0
for line in sys.stdin:
    left, right = line.strip().split(" | ")
    inputs = left.split()
    assert len(inputs) == 10
    outputs = right.split()
    
    value = {}
    configuration = {}
    for comb in inputs.copy():
        length = len(comb)
        if length == 2:
            correct = 1
        elif length == 3:
            correct = 7
        elif length == 4:
            correct = 4
        elif length == 7:
            correct = 8
        else:
            continue

        config = frozenset(comb)
        value[config] = correct
        configuration[correct] = config
        inputs.remove(comb)
    assert len(inputs) == 6

    for comb in inputs.copy():
        length = len(comb)
        config = frozenset(comb)
        if length == 5 and configuration[1].issubset(config):
            correct = 3
        elif length == 6 and configuration[4].issubset(config):
            correct = 9
        else:
            continue

        configuration[correct] = config
        value[config] = correct
        inputs.remove(comb)
    assert len(inputs) == 4

    for comb in inputs.copy():
        length = len(comb)
        config = frozenset(comb)
        if length == 6 and configuration[7].issubset(config):
            correct = 0
        else:
            continue

        configuration[correct] = config
        value[config] = correct
        inputs.remove(comb)
    assert len(inputs) == 3

    for comb in inputs.copy():
        length = len(comb)
        config = frozenset(comb)
        if length == 6:
            correct = 6
        else:
            continue

        configuration[correct] = config
        value[config] = correct
        inputs.remove(comb)
    assert len(inputs) == 2

    for comb in inputs.copy():
        length = len(comb)
        config = frozenset(comb)
        if length == 5 and config.issubset(configuration[6]):
            correct = 5
        else:
            correct = 2
        configuration[correct] = config
        value[config] = correct
        inputs.remove(comb)
    assert len(inputs) == 0

    current = 0
    for o in outputs:
        output_digit = value[frozenset(o)]
        current = current * 10 + output_digit

    sol += current

print(sol)
