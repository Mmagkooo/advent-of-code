from sys import argv

if len(argv) != 3:
    print("sol.py <N> <M>")
    exit()

N, M = map(int, argv[1:])

Q = 23
LS = 7

index = 0
player = -1
score = [0]*N
L = [0]
for marble in range(1, M+1):
    if (marble % 100000) == 0:
        print(marble)
    length = len(L)
    player = (player + 1) % N
    if marble % Q == 0:
        score[player] += marble
        index = ((index-7) % length + length) % length
        score[player] += L.pop(index)
    else:
        index = (index+2) % length
        #if index == 0: index = length
        L.insert(index, marble)
    
print(max(score))