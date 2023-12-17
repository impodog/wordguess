import re

def check(word: str) -> bool:
    for c in word:
        if not c.isalpha():
            return False
    return True

def output(s: str, name: str):
    res = list(map(lambda _: list(), range(3, 100)))

    for word in s.split("\n"):
        if not check(word):
            continue
        res[len(word)].append(word.lower())

    with open(f"assets/{name}.toml", "w") as f:
        f.write(f"words = [\n")
        for i in range(3, 100):
            if len(res[i]) > 0:
                f.write("[")
                for word in res[i]:
                    f.write("\"" + word + "\",")
                f.write("],\n")
            else:
                break
        f.write("]")

with open("assets/5000.txt", "r") as f:
    content = f.read()

l = content.split("\n")
res = set()

for i in range(len(l)):
    l[i] = re.sub(r"[a-z]+\.", r"", l[i])
    l[i] = re.sub(r"[A-C]\d", r"", l[i])
    l[i] = re.sub(r"\s+", r" ", l[i])
    if len(l[i]) > 0:
        res.add(l[i])

s = "\n".join(res)
s = re.sub(r"\s+n\s+", r" ", s)
s = re.sub(r"\,", r"", s)
s = re.sub(r"\s+\n", r"\n", s)
s = s.strip()

output(s, "common")

with open("assets/words.txt", "r") as f:
    s = f.read()
    
s = s.strip()

output(s, "words")