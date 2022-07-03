#%%
from utils import read

raw = read("../../data/2021/day10.txt")
ex = """[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
""".splitlines()
# raw = ex

# %%
checker = {"{": "}", "[": "]", "(": ")", "<": ">"}
table = {")": 3, "]": 57, "}": 1197, ">": 25137}
ats = {"(": 1, "[": 2, "{": 3, "<": 4}


def check(prev: str, curr: str) -> bool:
    corr = checker[prev]
    return corr == curr


points = 0
autocomp = []
for line in raw:
    curr = []
    for c in line:
        if c in ["{", "[", "(", "<"]:
            curr.append(c)
            continue

        if check(curr[-1], c):
            curr.pop()
        else:
            points += table[c]
            break
    else:  # Incomplete lines
        p = 0
        for c in reversed(curr):
            p *= 5
            p += ats[c]
        autocomp.append(p)

print(points, sorted(autocomp)[len(autocomp) // 2])
# %%
