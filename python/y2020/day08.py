#%%
import copy

from utils import load

raw = load("day08.txt", split="\n")

Proc = list[tuple[str, int]]
proc: Proc = [((u := x.split())[0], int(u[1])) for x in raw]

# %%
def execute(program: Proc, acc: int, curr: int) -> tuple[int, int]:
    cmd, n = program[curr]
    if cmd == "acc":
        acc += n
        curr += 1
    elif cmd == "jmp":
        curr += n
    elif cmd == "nop":
        curr += 1
    else:
        raise ValueError("WTF")
    return acc, curr


def run(program: Proc, acc: int = 0, curr: int = 0) -> tuple[int, bool]:
    passed = set()
    while True:
        if curr in passed:
            break
        else:
            passed.add(curr)
        if curr == len(program):
            return acc, True

        acc, curr = execute(program, acc, curr)
    return acc, False


def test1() -> None:
    assert run(proc)[0] == 1684


# %%


def swap(test: Proc, loc: int) -> None:
    if test[loc][0] == "jmp":
        test[loc] = ("nop", test[loc][1])
    elif test[loc][0] == "nop":
        test[loc] = ("jmp", test[loc][1])
    else:
        raise ValueError


def test2() -> None:
    passed = set()
    test = copy.deepcopy(proc)
    acc = 0
    curr = 0
    while True:
        if curr in passed:
            raise ValueError
        passed.add(curr)

        # Change
        if proc[curr][0] in ["jmp", "nop"]:
            swap(test, curr)
        else:
            acc, curr = execute(proc, acc, curr)
            continue

        # Test
        out, ok = run(test, acc, curr)
        if ok:
            assert out == 2188
            return
        else:
            swap(test, curr)
            acc, curr = execute(test, acc, curr)


# %%

# %%
