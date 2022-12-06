L = list(map(int, input().split()))        
def search(i, n):
    s = 0
    for _ in range(n):
        nchildren, nmeta = L[i], L[i+1]
        i, currs = search(i+2, nchildren)
        s += currs
        for meta in range(nmeta):
            s += L[i]
            i += 1
    return i, s
print(search(0,1)[1])