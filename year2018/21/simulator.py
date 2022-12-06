R0, R1, R2, R3, R4, R5 = 5876609, 0, 0, 0, 0, 0
def my_print():
    print(R0, R1, R2, R3, R4, R5)

while 123 & 456 != 72:
    pass
memo = set()
last = None
R5 = 0
R3 = R5 | 65536
R5 = 521363
while True:
    #my_print()
    R5 += (R3 & 255)
    R5 &= 16777215
    R5 *= 65899
    R5 &= 16777215
    if R3 < 256:
        if R5 in memo:
            print("last:", last)
            break
        #memo.add(R5)
        last = R5
        if R5 == R0:
            break
        R3 = R5 | 65536
        R5 = 521363
    else:
        R4 = 0
        while True:
            R2 = (R4+1)*256
            if R2 > R3: break
            R4 += 1
        
        R3 = R4