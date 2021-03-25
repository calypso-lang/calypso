import random

binops = [
    "+", "-", "*", "/", "%", "**", "||", "&&", "|", "^", "&", "<<", ">>", "==",
    "!=", "<", ">", "<=", ">="
]

unops = [ "!", "-", "" ]

s = ""

for i in range(1, 100):
    s += random.choice(unops)
    s += str(random.randrange(1, 1000))
    s += random.choice(binops)

s += str(random.randrange(1, 1000))

print(s)
