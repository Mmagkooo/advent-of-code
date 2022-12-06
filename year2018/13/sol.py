import sys

def pretty(m, carts):
    mapa = {}
    for i, j, c, cnt in carts:
        mapa[i,j] = c
    for i, row in enumerate(m):
        for j, c in enumerate(row):
            print(mapa.get((i,j), m[i][j]), end="")
        print()
m = []
for line in sys.stdin:
    m.append(list(line.strip('\n')))

cart_symbols = "^v<>"
carts = []
carts_set = set()
for i, row in enumerate(m):
    for j, c in enumerate(row):
        if c == '>' or c == '<':
            m[i][j] = '-'
        elif c == '^' or c == 'v':
            m[i][j] = '|'
        else:
            continue
        carts.append((i, j, c, 0))
        carts_set.add((i, j))

ok = True
while ok:
    ncarts = []
    ncarts_set = set()
    for index, cart in enumerate(carts):
        row, col, c, cnt = cart
        if c == '^':
            nc = m[row-1][col]
            if nc == '|':
                pass
            elif nc == '/':
                c = '>'
            elif nc == '\\':
                c = '<'
            elif nc == '+':
                if cnt == 0:
                    c = '<'
                elif cnt == 1:
                    c = '^'
                elif cnt == 2:
                    c = '>'
                cnt = (cnt+1) % 3
            
            ncart = row-1, col, c, cnt

        elif c == 'v':
            nc = m[row+1][col]
            if nc == '|':
                pass
            elif nc == '/':
                c = '<'
            elif nc == '\\':
                c = '>'
            elif nc == '+':
                if cnt == 0:
                    c = '>'
                elif cnt == 1:
                    c = 'v'
                elif cnt == 2:
                    c = '<'
                cnt = (cnt+1) % 3
            
            ncart = row+1, col, c, cnt
            
        elif c == '>':
            nc = m[row][col+1]
            if nc == '-':
                pass
            elif nc == '/':
                c = '^'
            elif nc == '\\':
                c = 'v'
            elif nc == '+':
                if cnt == 0:
                    c = '^'
                elif cnt == 1:
                    c = '>'
                elif cnt == 2:
                    c = 'v'
                cnt = (cnt+1) % 3
            
            ncart = row, col+1, c, cnt

        elif c == '<':
            nc = m[row][col-1]
            if nc == '-':
                pass
            elif nc == '/':
                c = 'v'
            elif nc == '\\':
                c = '^'
            elif nc == '+':
                if cnt == 0:
                    c = 'v'
                elif cnt == 1:
                    c = '<'
                elif cnt == 2:
                    c = '^'
                cnt = (cnt+1) % 3
            
            ncart = row, col-1, c, cnt
        
        ntuple = (ncart[0], ncart[1])
        if ntuple in carts_set or ntuple in ncarts_set:
            print(ntuple,sep=",")
            ok = False
            break
        carts[index] = None
        carts_set.remove((row, col))
        ncarts.append(ncart)
        ncarts_set.add(ntuple)

    carts = sorted(ncarts)
    carts_set = ncarts_set
