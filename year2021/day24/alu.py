opcode2op = {
    "add": "+",
    "mul": "*",
    "div": "//",
    "mod": "%",
    "eql": "==",
}

def alu(instructions, input):
    assert isinstance(input, list)
    assert all(map(lambda i: isinstance(i, str), input))

    iter = input.__iter__()
    memory = { "w": 0, "x": 0, "y": 0, "z": 0 }
    for instruction in instructions:
        opcode, storage, *args = instruction.strip("\n").split()

        if opcode == "inp":
            input_value = next(iter)
            py_code = f"{storage} = {input_value}"
        else:
            py_code = f"{storage} = int({storage} {opcode2op[opcode]} {args[0]})"
        exec(py_code, None, memory)
    
    try:
        next(iter)
        raise Exception("There is unused input")
    except StopIteration:
        pass
    return memory

def get_instructions(path):
    with open(path) as f:
        return f.readlines()