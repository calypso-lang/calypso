import random
import sys

nops = int(sys.argv[1] or "100")
nmax = int(sys.argv[2] or "1000")

binops = [
    "+", "-", "*", "/", "%", "**", "||", "&&", "|", "^", "&", "<<", ">>", "==",
    "!=", "<", ">", "<=", ">="
]

unops = [ "!", "-", "" ]

s = ""

for i in range(1, nops):
    s += random.choice(unops)
    s += str(random.randrange(0, nmax))
    s += random.choice(binops)

s += str(random.randrange(0, nmax))

print(s)
