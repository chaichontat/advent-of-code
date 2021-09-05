#%%
import re

from utils import load

cmds, msgs = [x.strip().replace('"', "").split("\n") for x in load("day19.txt", split="\n\n")]
cmds = {int((u := cmd.split(": "))[0]): u[1] for cmd in cmds}

# %%
def recurse(cmds: dict[int, str], n: int) -> str:
    if cmds[n].startswith("override"):
        return cmds[n][8:]

    toadd = ""

    groups = [x.split(" ") for x in cmds[n].split(" | ")]
    for i, group in enumerate(groups):
        if len(groups) > 1:
            toadd += "("

        for cmd in group:
            if len(group) > 1:
                toadd += "("
            try:
                toadd += recurse(cmds, int(cmd))
            except ValueError:
                toadd += cmd
            if len(group) > 1:
                toadd += ")"

        if len(groups) > 1:
            toadd += ")"
        if i < len(groups) - 1:
            toadd += "|"

    return toadd


#%%
def test1() -> None:
    assert sum(bool(re.fullmatch(recurse(cmds, 0), msg)) for msg in msgs) == 192


# %%
def test2() -> None:
    cmds2 = cmds.copy()
    cmds2[8] = f"override({recurse(cmds, 42)})+"
    cmds2[11] = (
        "override("
        + "|".join(f"({recurse(cmds, 42)}){{{i}}}({recurse(cmds, 31)}){{{i}}}" for i in range(1, 6))
        + ")"
    )
    assert sum(bool(re.fullmatch(recurse(cmds2, 0), msg)) for msg in msgs) == 296


# %%
