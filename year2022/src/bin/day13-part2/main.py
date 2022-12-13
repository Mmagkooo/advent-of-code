import sys
from typing import List
from functools import cmp_to_key


def compare(left: List, right: List):
    for left_el, right_el in zip(left, right):
        left_el_is_int = isinstance(left_el, int)
        right_el_is_int = isinstance(right_el, int)
        if left_el_is_int and right_el_is_int:
            if left_el < right_el:
                return -1
            if left_el > right_el:
                return 1
        elif not left_el_is_int and not right_el_is_int:
            list_comparison = compare(left_el, right_el)
            if list_comparison is not None:
                return list_comparison
        else:
            list_comparison = (
                compare([left_el], right_el)
                if left_el_is_int
                else compare(left_el, [right_el])
            )
            if list_comparison is not None:
                return list_comparison
    
    if len(left) < len(right):
        return -1
    elif len(left) > len(right):
        return 1

    # lists are equal
    return 0

def main():
    lines = [eval(line.strip()) for line in sys.stdin if line.strip()]

    divider_packet1 = [[2]]
    divider_packet2 = [[6]]
    lines.append(divider_packet1)
    lines.append(divider_packet2)

    sorted_lines = sorted(lines, key=cmp_to_key(compare))
    sol = (sorted_lines.index(divider_packet1) + 1) * (sorted_lines.index(divider_packet2) + 1)
    print(sol)

if __name__ == "__main__":
    main()
