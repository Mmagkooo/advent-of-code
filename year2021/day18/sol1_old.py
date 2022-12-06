import sys

LEFT = 0
RIGHT = 1
MAX_LEVEL = 3

def split(number, parent):
    left = number // 2
    right = number // 2 + number % 2
    return SnailfishNumber([left, right], parent)

def get_member_magnitude(member):
    return member if isinstance(member, int) else member.get_magnitude()

def explode_leftmost_too_deep_member(member, level):
    if isinstance(member, int):
        return False
    return member.explode_leftmost_too_deep(level=level+1)

class SnailfishNumber:
    def __init__(self, material, parent):
        assert len(material) == 2
        print("DEBUG init", material, parent)
        self.left = self.create_member(material[0])
        self.right = self.create_member(material[1])
        self.parent = parent

    def __str__(self):
        return f"[{str(self.left)}, {str(self.right)}, parent={self.parent is not None}]"
    
    def __repr__(self):
        return str(self)

    def create_member(self, material):
        if isinstance(material, list):
            return SnailfishNumber(material, self)
        if isinstance(material, SnailfishNumber):
            material.parent = self
            return material
        return material

    def reduce(self):
        assert self.parent is None
        print("before reduce", self)
        while True:
            if self.explode_leftmost_too_deep():
                continue
            if self.split_leftmost():
                continue
            break
        print("DEBUG after reduce", self)

    def explode_leftmost_too_deep(self, level=0):
        if level == MAX_LEVEL:
            print("DEBUG exploding", self)
            self.explode()
            return True

        ret_left = explode_leftmost_too_deep_member(self.left, level=level)
        if ret_left:
            return ret_left
        return explode_leftmost_too_deep_member(self.right, level=level)

    def split_leftmost(self):
        if isinstance(self.left, int):
            if self.left >= 10:
                self.left = split(self.left)
                return True

        if isinstance(self.left, SnailfishNumber) and self.left.split_leftmost():
            return True

        if isinstance(self.right, int):
            if self.right >= 10:
                return True

        return isinstance(self.right, SnailfishNumber) and self.right.split_leftmost()

    def explode(self):
        if self.parent is None:
            return
        self.parent.receive_explosion(self.left, LEFT, received_from=self, first_call=True)
        self.parent.receive_explosion(self.right, RIGHT, received_from=self, first_call=True)

    def receive_explosion(self, number, exploded_member, received_from, first_call=False):
        if exploded_member == LEFT:
            if first_call:
                self.left = 0
            elif received_from is self.left:
                self.explode_down(number, RIGHT)
            elif received_from is self.right:
                self.explode_down(number, LEFT)
            else:
                raise ValueError(f"received_from={received_from}")
        elif exploded_member == RIGHT:
            if first_call:
                self.right = 0
            elif received_from is self.left:
                self.explode_down(number, RIGHT)
            elif received_from is self.right:
                self.explode_down(number, LEFT)
            else:
                raise ValueError(f"received_from={received_from}")
        else:
            raise ValueError(f"exploded_member={exploded_member}")
        
        if self.parent:
            self.parent.receive_explosion(number, exploded_member, self)
    
    def explode_down(self, number, target_member):
        if target_member == LEFT:
            if isinstance(self.left, int):
                self.left += number
            else:
                self.left.explode_down(number, target_member)
        elif target_member == RIGHT:
            if isinstance(self.right, int):
                self.right += number
            else:
                self.right.explode_down(number, target_member)
        else:
            raise ValueError(f"target_member={target_member}")

    def __add__(self, other):
        print("DEBUG adding", self, other)
        ret = SnailfishNumber([self, other], None)
        print("DEBUG reducing", ret)
        ret.reduce()
        return ret

    def get_magnitude(self):
        return 3 * get_member_magnitude(self.left) + 2 * get_member_magnitude(self.right)

numbers = [SnailfishNumber(eval(line.strip()), None) for line in sys.stdin]
final_number = numbers[0]
for addition in numbers[1:]:
    final_number = final_number + addition

print(final_number.get_magnitude())
