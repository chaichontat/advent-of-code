from pathlib import Path
from typing import Literal, overload

import numpy as np


@overload
def read(path: str, *, raw: Literal[True] = ...) -> str:
    ...


@overload
def read(path: str, *, parse: Literal[True] = ...) -> np.ndarray:
    ...


@overload
def read(path: str, *, parse: Literal[False] = ...) -> list[str]:
    ...


def read(path, raw: bool = False, parse: bool = False) -> np.ndarray | list[str] | str:
    lines = (r := Path(path).read_text()).splitlines()
    if raw:
        return r
    if parse:
        return np.array(list(map(int, lines)))
    return lines
