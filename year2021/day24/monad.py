from alu import alu, get_instructions

def monad(number):
    ret = alu(get_instructions("day24/monad.txt"), list(number))
    return ret["z"]

if __name__ == "__main__":
    input_number = input()
    print(monad(input_number))
