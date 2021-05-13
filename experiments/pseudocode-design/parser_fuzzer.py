import random
import string
# import sys

# nops = int(sys.argv[1] or "100")
# nmax = int(sys.argv[2] or "1000")
# ident_len = int(sys.argv[3] or "16")
# astr_len = int(sys.argv[4] or "16")

BINOPS = [
    "+", "-", "*", "/", "%", "**", "||", "&&", "|", "^", "&", "<<", ">>", "==",
    "!=", "<", ">", "<=", ">="
]

UNOPS = [ "!", "-", "" ]

IDENT_START = string.ascii_letters
IDENT_CONTINUE = IDENT_START + string.digits + "_"

s = ""

INT_MAX = 255

def gen_integer():
    return str(random.randrange(0, INT_MAX + 1)) + random.choice(["s", "u", "f", ""])

def gen_float():
    return str(random.uniform(0, 1))

def gen_bool():
    return random.choice(["true", "false"])

def gen_literal():
    ops = [
        gen_integer,
        gen_bool,
        gen_float
    ]
    return random.choice(ops)()

def gen_unop():
    return random.choice(UNOPS)

def gen_binop():
    return random.choice(BINOPS)

def gen_expr(standalone = True):
    expr = gen_unop() + gen_literal() + gen_binop()
    if standalone:
        expr += gen_literal()
    return expr

def gen_exprs(n_iters = 1024):
    return ''.join(gen_expr(False) for _ in range(n_iters)) + gen_expr(True)

def get(l, i):
    return l[i] if i < len(l) else None

if __name__ == "__main__":
    import sys
    n_iters = int(get(sys.argv, 1) or "1024")
    INT_MAX = int(get(sys.argv, 2) or "255")
    print(gen_exprs(n_iters))
