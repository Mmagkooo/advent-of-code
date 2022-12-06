import sys
import numpy as np
from transformations import transformations

MATCH_THRESHOLD = 12

def match(scanner1, scanner2):
    scanner1_set = set(map(tuple, scanner1))
    for reading1 in scanner1:
        for reading2 in scanner2:
            diff = reading1 - reading2
            scanner2_adjusted = scanner2 + diff
            scanner2_adjusted_set = set(map(tuple, scanner2_adjusted))
            intersection = scanner1_set.intersection(scanner2_adjusted_set)
            if len(intersection) >= MATCH_THRESHOLD:
                union = scanner1_set.union(scanner2_adjusted_set)
                return np.array(list(union)), diff
    return None, None

def read_scanners():
    scanner_inputs = sys.stdin.read().strip().split("\n\n")
    scanners = []
    for scanner_input in scanner_inputs:
        scanner_readings = scanner_input.split("\n")[1:]
        scanner_rows = []
        for row in scanner_readings:
            scanner_rows.append(np.array([int(r) for r in row.split(",")]))
        scanners.append(np.array(scanner_rows))
    return scanners

def main():
    scanners = read_scanners()
    scanner = scanners[0]
    remaining_scanners = scanners[1:]

    scanner_positions = []
    while remaining_scanners:
        new_remaining_scanners = []
        for remaining_scanner in remaining_scanners:
            for transformation in transformations:
                remaining_scanner_transformed = np.matmul(remaining_scanner, transformation)
                new_scanner, scanner_position = match(scanner, remaining_scanner_transformed)
                if new_scanner is not None:
                    scanner = new_scanner
                    scanner_positions.append(scanner_position)
                    break
            else:
                new_remaining_scanners.append(remaining_scanner)

        new_remaining_number = len(new_remaining_scanners)
        old_remaining_number = len(remaining_scanners)
        assert new_remaining_number < old_remaining_number, f"old={old_remaining_number}, new={new_remaining_number}"
        remaining_scanners = new_remaining_scanners

    max_dist = 0
    for pos1 in scanner_positions:
        for pos2 in scanner_positions:
            dist = np.abs(pos1 - pos2).sum()
            max_dist = max(max_dist, dist)
    print(max_dist)

if __name__ == "__main__":
    main()
