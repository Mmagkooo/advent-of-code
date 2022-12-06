class Pawn:
    THROWS_IN_STEP = 3
    MAX_POS = 10
    def __init__(self, pos):
        self.pos = pos
        self.score = 0

    def move(self, die):
        for _ in range(Pawn.THROWS_IN_STEP):
            self.pos += die.roll()
        self.pos %= Pawn.MAX_POS
        if self.pos == 0:
            self.pos = Pawn.MAX_POS
        self.score += self.pos

    def __str__(self):
        return f"Pawn<pos={self.pos}, score={self.score}>"

    def __repr__(self):
        return str(self)

class Die:
    MAX_NUMBER = 100
    def __init__(self):
        self.number = 1
        self.rolls = 0
    
    def roll(self):
        ret = self.number
        self.number += 1
        if self.number > Die.MAX_NUMBER:
            self.number = 1
        
        self.rolls += 1
        return ret

    def __str__(self):
        return f"Die<number={self.number}, rolls={self.rolls}>"

    def __repr__(self):
        return str(self)

pawn1, pawn2 = [Pawn(int(pos_str.split()[-1])) for pos_str in [input(), input()]]
die = Die()

SCORE_LIMIT = 1000
while True:
    pawn1.move(die)
    if pawn1.score >= SCORE_LIMIT:
        break

    pawn2.move(die)
    if pawn2.score >= SCORE_LIMIT:
        break

print(min(pawn1.score, pawn2.score) * die.rolls)
