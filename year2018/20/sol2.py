def mul_list(l1, l2):
    ret = []
    for word1 in l1:
        for word2 in l2:
            ret.append(word1 + word2)
    return ret

def rek(i):
    l = [[""]]
    word = []
    while i < len(L) and L[i] != ')':
        curr = L[i]
        if curr in "NWSE":
            word.append(curr)
        elif curr == '|':
            l.append(word)
            word = []
        elif curr == '(':
            #print("".join(word))
            l = mul_list(l, [word])
            word = []
            other, i = rek(i+1)
            #print(other)
            l = mul_list(l, other)
        i += 1
    l = mul_list(l, [word])
    return l, i

L = input()[1:-1]
l, _ = rek(0)
for word in l:
    print("".join(word))