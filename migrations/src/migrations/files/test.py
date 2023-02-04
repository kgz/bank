import random


roles = {
    "create_users" : 0, 
    "delete_users" : 1,
    "run_migrations" : 1,
    "see_migrations" : 1,
    "delete_users" : 0,
    "enable_users" : 1,
    "view_user_settings" : 0,
    "view_app_settings" : 1,
    "change_permissions" : 1,
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
    # getamount to add to round tonearest 8
    add = 8 - (len(code) % 8)

    code = str(code).zfill(len(code) + add - 1)
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
    return tre




code = generate_code()
print(code)
r = get_roldes_from_code(code)
print(r)

