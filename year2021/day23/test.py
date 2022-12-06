from dataclasses import dataclass

@dataclass(frozen=True, eq=True)
class Location:
    room: str
    index: int

a = Location("H", 1)
b = Location("H", 1)
c = Location("H", 2)

print(a == b)
print(a == c)

print(set([a, b, c]))

a.room = "A"