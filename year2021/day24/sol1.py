import random
import sys
from monad import monad

def get_random_numstring(length=14):
    return "".join(str(random.randint(1, 9)) for _ in range(length))

# for number in range(99599999999999, 99599900000000 - 1, -1):
#     number = str(number)
#     if "0" in number:
#         continue
#     res = monad(number)
#     if res == 0:
#         print(number)

