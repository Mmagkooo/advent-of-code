z0 = 0
read_input = lambda: None

w1 = read_input()
z1 = z0 // 1
if (z0 % 26) + 14 != w1:
    z1 = 26 * z1 + w1 + 7

w2 = read_input()
z2 = z1 // 1
if (z1 % 26) + 12 != w2:
    z2 = 26 * z2 + w2 + 4

w3 = read_input()
z3 = z2 // 1
if (z2 % 26) + 11 != w3:
    z3 = 26 * z3 + w3 + 8

w4 = read_input()
z4 = z3 // 26
if (z3 % 26) + -4 != w4:
    z4 = 26 * z4 + w4 + 1

w5 = read_input()
z5 = z4 // 1
if (z4 % 26) + 10 != w5:
    z5 = 26 * z5 + w5 + 5

w6 = read_input()
z6 = z5 // 1
if (z5 % 26) + 10 != w6:
    z6 = 26 * z6 + w6 + 14

w7 = read_input()
z7 = z6 // 1
if (z6 % 26) + 15 != w7:
    z7 = 26 * z7 + w7 + 12

w8 = read_input()
z8 = z7 // 26
if (z7 % 26) + -9 != w8:
    z8 = 26 * z8 + w8 + 10

w9 = read_input()
z9 = z8 // 26
if (z8 % 26) + -9 != w9:
    z9 = 26 * z9 + w9 + 5

w10 = read_input()
z10 = z9 // 1
if (z9 % 26) + 12 != w10:
    z10 = 26 * z10 + w10 + 7

w11 = read_input()
z11 = z10 // 26
if (z10 % 26) + -15 != w11:
    z11 = 26 * z11 + w11 + 6

w12 = read_input()
z12 = z11 // 26
if (z11 % 26) + -7 != w12:
    z12 = 26 * z12 + w12 + 8

w13 = read_input()
z13 = z12 // 26
if (z12 % 26) + -10 != w13:
    z13 = 26 * z13 + w13 + 4

w14 = read_input()
z14 = z13 // 26
if (z13 % 26) + 0 != w14:
    z14 = 26 * z14 + w14 + 6
