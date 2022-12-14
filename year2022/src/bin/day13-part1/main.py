import sys
from typing import List


def compare(left: List, right: List):
    for left_el, right_el in zip(left, right):
        left_el_is_int = isinstance(left_el, int)
        right_el_is_int = isinstance(right_el, int)
        if left_el_is_int and right_el_is_int:
            if left_el != right_el:
                return left_el - right_el
        else:
            if left_el_is_int:
                left_el = [left_el]
            if right_el_is_int:
                right_el = [right_el]

            list_comparison = compare(left_el, right_el)
            if list_comparison:
                return list_comparison

    return len(left) - len(right)


def main():
    lines = [line.strip() for line in sys.stdin]

    sol = 0
    for line_i in range(0, len(lines), 3):
        left = eval(lines[line_i])
        right = eval(lines[line_i + 1])

        if compare(left, right) < 0:
            i = line_i // 3 + 1
            print("Correct index:", i)
            sol += i

    print(sol)


if __name__ == "__main__":
    main()
