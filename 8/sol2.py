L = list(map(int, input().split()))

def search(i, n):
    vals = []
    for _ in range(n):
        nchildren, nmeta = L[i], L[i+1]
        i, currvals = search(i+2, nchildren)
        
        val = 0
        
        if nchildren == 0:            
            for meta in range(nmeta):
                val += L[i]
                i += 1
            
        else:
            for meta in range(nmeta):
                index = L[i]-1
                if 0 <= index and index < nchildren:
                    val += currvals[index]
                i += 1
        
        vals.append(val)
    
    return i, vals

_, vals = search(0, 1)
print(sum(vals))