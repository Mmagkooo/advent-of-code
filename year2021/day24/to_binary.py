from alu import alu, get_instructions

instructions = get_instructions("day24/to_binary.txt")

def to_binary(number):
    ret = alu(instructions, [number])
    return f'{ret["w"]}{ret["x"]}{ret["y"]}{ret["z"]}'

if __name__ == "__main__":
    print(to_binary(input()))
