hex2bin = {}
for c in "0123456789ABCDEF":
    hex2bin[c] = "{0:04b}".format(int(c, 16))

def prod(arr):
    ret = 1
    for el in arr:
        ret *= el
    return ret

code2op = {
    0: sum,
    1: prod,
    2: min,
    3: max,
    5: lambda packets: int(packets[0] > packets[1]),
    6: lambda packets: int(packets[0] < packets[1]),
    7: lambda packets: int(packets[0] == packets[1])
}

def empty_line(line):
    if not line:
        return True
    return "0" in line and len(set(line)) == 1

def parse(line):
    print()
    if empty_line(line):
        return 0, ""
    ver = int(line[:3], 2)
    line = line[3:]

    type = int(line[:3], 2)
    line = line[3:]
    print("type", type)

    ## literal
    if type == 4:
        value = ""
        while True:
            segment = line[:5]
            line = line[5:]
            value += segment[1:]
            if segment[0] == "0":
                break
        value = int(value, 2)
        print("ret", value)
        print("ret_rest", line)
        return value, line
    
    ## otherwise operator
    length_type = int(line[0], 2)
    line = line[1:]
    print("length_type", length_type)
    if length_type == 0:
        print("line before split", line)
        length = int(line[:15], 2)
        line = line[15:]
        print("line after split", line)
        print("length", length)
        subpacket_material = line[:length]
        print("subpacket_material", line)
        ret_rest = line[length:]
        values = []
        while not empty_line(subpacket_material):
            sub_value, subpacket_material = parse(subpacket_material)
            values.append(sub_value)
        ret = code2op[type](values)
        print("ret", ret)
        print("ret_rest", ret_rest)
        return ret, ret_rest

    ## otherwise length_type == 1
    number_of_subpackets = int(line[:11], 2)
    line = line[11:]
    print("subpackets", number_of_subpackets)
    values = []
    for _ in range(number_of_subpackets):
        sub_value, line = parse(line)
        values.append(sub_value)
    ret = code2op[type](values)
    print("ret", ret)
    print("ret_rest", line)
    return ret, line

original_binary = "".join([hex2bin[c] for c in input()])
res, _ = parse(original_binary)
print(res)
