import sys
from copy import deepcopy

LEFT = 0
RIGHT = 1
MAX_LEVEL = 5
SPLIT_THRESHOLD = 10

def create_number(material, parent):
    if isinstance(material, list):
        return SnailfishNumber(material, parent)
    if isinstance(material, SnailfishNumber):
        material.parent = parent
        return material
    return RegularNumber(material, parent)

class RegularNumber:
    def __init__(self, value, parent):
        self.value = value
        self.parent = parent
    
    def get_magnitude(self):
        return self.value
    
    def __str__(self):
        return str(self.value)
    
    def __repr__(self):
        return str(self)

    def explode(self, level):
        return False

    def explode_down(self, number, target_member):
        self.value += number
        return True

    def split(self):
        if self.value < SPLIT_THRESHOLD:
            return False

        left = self.value // 2
        right = self.value // 2 + self.value % 2
        new_number = SnailfishNumber([left, right], self.parent)
        if self.parent.left is self:
            self.parent.left = new_number
        else:
            self.parent.right = new_number
        return True

class SnailfishNumber:
    def __init__(self, material, parent):
        assert len(material) == 2
        self.left = create_number(material[0], self)
        self.right = create_number(material[1], self)
        self.parent = parent

    def __str__(self):
        return f"[{str(self.left)}, {str(self.right)}]"
    
    def __repr__(self):
        return str(self)

    def reduce(self):
        assert self.parent is None
        while True:
            if self.explode():
                continue
            if self.split():
                continue
            break

    def find_good_ancestor(self, needed_side):
        prev_level = self
        ancestor = self.parent
        while ancestor:
            if needed_side == RIGHT:
                if prev_level is ancestor.left:
                    break
            elif needed_side == LEFT:
                if prev_level is ancestor.right:
                    break
            prev_level = ancestor
            ancestor = ancestor.parent
        return ancestor

    def split(self):
        return self.left.split() or self.right.split()

    def explode(self, level=1):
        if level == MAX_LEVEL:
            right_ancestor = self.find_good_ancestor(RIGHT)
            if right_ancestor:
                right_ancestor.right.explode_down(self.right.value, LEFT)

            left_ancestor = self.find_good_ancestor(LEFT)
            if left_ancestor:
                left_ancestor.left.explode_down(self.left.value, RIGHT)

            new_number = RegularNumber(0, self.parent)
            if self.parent.left is self:
                self.parent.left = new_number
            elif self.parent.right is self:
                self.parent.right = new_number
            else:
                raise ValueError

            return True
        return self.left.explode(level+1) or self.right.explode(level+1)

    def explode_down(self, number, target_member):
        if target_member == LEFT:
            return self.left.explode_down(number, target_member)
        elif target_member == RIGHT:
            return self.right.explode_down(number, target_member)
        else:
            raise ValueError(f"target_member={target_member}")

    def __add__(self, other):
        ret = SnailfishNumber([deepcopy(self), deepcopy(other)], None)
        ret.reduce()
        return ret

    def get_magnitude(self):
        return 3 * self.left.get_magnitude() + 2 * self.right.get_magnitude()

numbers = [SnailfishNumber(eval(line.strip()), None) for line in sys.stdin]
final_number = numbers[0]
for addition in numbers[1:]:
    final_number = final_number + addition

print(final_number.get_magnitude())
