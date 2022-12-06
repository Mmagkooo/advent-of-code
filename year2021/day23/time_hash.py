class Config:
    def __init__(self, hallway, room_a, room_b, room_c, room_d):
        self.hallway = hallway
        self.room_a = room_a
        self.room_b = room_b
        self.room_c = room_c
        self.room_d = room_d
        self.hash = hash((hallway, room_a, room_b, room_c, room_d))

    def __hash__(self):
        return self.hash

memo = {}

config = ((None,) * 11, ("A", "A"), ("B", "B"), ("C", "C"), ("D", "D"))
#config = Config(*config)
memo[config] = "ok"
cnt = 0
for _ in range(int(1e9)):
    if config in memo:
        cnt += 1
print(cnt)