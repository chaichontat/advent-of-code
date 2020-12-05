from pathlib import Path
from typing import List


def load(path: str, parseint=False, split="\n") -> List:
    raw = Path(path).read_text().split(split)
    if parseint:
        return [int(x) for x in raw]
    return raw
