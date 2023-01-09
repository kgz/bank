import random


roles = {
    "a" : 0, 
    "b" : 1,
    "c" : 1,
    "d" : 1,
    "e" : 0,
    "f" : 1,
    "g" : 0,
    "h" : 1,
    "i" : 1,
    "j" : 1,
    "k" : 0,
}


def generate_code():
    values = list(roles.values())
    print(values)
    code = ""
    last = None
    for x in values:
        if last is None:
            last = x
            code += str(x)
            continue
        if last == x:
            code += str(0)
            continue
        else:
            code += str(1)
            last = x
    
    code = str(code)
    code = str(code).zfill(7)
    code = "1" + code
    code = hex(int(code, 2))
    code = code[2:]
    return code

def reverse_code(code):
    code = "0x" + code
    code = int(code, 16)
    code = bin(code)[2:] 
    code = str(code)[1:]
    ncode = str(code[0])

    for i in code[1:]:      
        if (ncode[-1] == "0") and (i == "0"):
            ncode += "0"
            continue
        if (ncode[-1] == "1") and (i == "1"):
            ncode += "0"
            continue
        if (ncode[-1] == "1") and (i == "0"):
            ncode += "1"
            continue
        if (ncode[-1] == "0") and (i == "1"):
            ncode += "1"
            continue
    return ncode


def get_roldes_from_code(code):
    bools = reverse_code(code)

    tre = []
    for x, item in enumerate(roles):
        if(bools[x] == "1"):
            tre.append(item)
    print(tre)




code = generate_code()
print(code)
get_roldes_from_code(code)


