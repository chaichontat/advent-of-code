from pathlib import Path
from typing import List


def load(path: str, parseint=False) -> List:
    raw = Path(path).read_text().split()
    if parseint:
        return [int(x) for x in raw]
    return raw
