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

def get_prev_pos(curr_pos, roll):
    ret = (curr_pos - roll) % MAX_POS
    if ret == 0:
        ret = MAX_POS
    return ret

memo = {}
def count(pos1, pos2, score1, score2, last_played): # turn is either 1 or 2
    if score1 < 0 or score2 < 0:
        return 0

    # last_played == 2 means it's currently 1's turn
    if pos1 == start_pos1 and pos2 == start_pos2 and score1 == score2 == 0 and last_played == 2:
        return 1

    key = pos1, pos2, score1, score2, last_played
    if key in memo:
        return memo[key]

    ret = 0
    for roll in ROLLS:
        ways = ROLLS[roll]
        prev_played = 3 - last_played
        if last_played == 1:
            prev_score1 = score1 - pos1
            if prev_score1 >= WIN_SCORE:
                continue
            curr_cnt = count(get_prev_pos(pos1, roll), pos2, prev_score1, score2, prev_played)
        else:
            prev_score2 = score2 - pos2
            if prev_score2 >= WIN_SCORE:
                continue
            curr_cnt = count(pos1, get_prev_pos(pos2, roll), score1, prev_score2, prev_played)
        ret += ways * curr_cnt

    memo[key] = ret
    return ret

for turn in [1, 2]:
    sol = 0
    for final_pos1 in range(1, MAX_POS + 1):
        for final_pos2 in range(1, MAX_POS + 1):
            for overkill in range(0, MAX_POS):
                winning_score = WIN_SCORE + overkill
                for losing_score in range(0, WIN_SCORE):
                    if turn == 1:
                        curr_cnt = count(final_pos1, final_pos2, winning_score, losing_score, 1)
                    else:
                        curr_cnt = count(final_pos1, final_pos2, losing_score, winning_score, 2)
                    sol += curr_cnt
    print(turn, sol)
