start_pos1, start_pos2 = [int(input().split()[-1]) for _ in range(2)]

ROLLS = {
    3: 1,
    4: 3,
    5: 6,
    6: 7,
    7: 6,
    8: 3,
    9: 1,
}

MAX_POS = 10
WIN_SCORE = 21

def get_next_pos(pos, roll):
    next_pos = (pos + roll) % MAX_POS
    if next_pos == 0:
        next_pos = MAX_POS
    return next_pos

sol1 = 0
sol2 = 0
def count(pos1, pos2, score1, score2, turn, accumulated):
    if score2 >= WIN_SCORE:
        global sol2
        sol2 += accumulated
        return

    if score1 >= WIN_SCORE:
        global sol1
        sol1 += accumulated
        return

    for roll in ROLLS:
        next_turn = 3 - turn
        next_accumulated = accumulated * ROLLS[roll]
        if turn == 1:
            next_pos1 = get_next_pos(pos1, roll)
            count(next_pos1, pos2, score1 + next_pos1, score2, next_turn, next_accumulated)
        else:
            next_pos2 = get_next_pos(pos2, roll)
            count(pos1, next_pos2, score1, score2 + next_pos2, next_turn, next_accumulated)

count(start_pos1, start_pos2, 0, 0, 1, 1)
print(sol1, sol2)
print(max(sol1, sol2))
