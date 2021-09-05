from __future__ import annotations  # Until Python 3.10

from pathlib import Path
from typing import Literal, Optional, Sequence, overload

import numpy as np
import numpy.typing as npt

NDArrayInt = npt.NDArray[np.int_]


@overload
def load(path: Path | str, *, parseint: Literal[True], split: Optional[str] = ...) -> list[int]:
    ...


@overload
def load(path: Path | str, *, parseint: Literal[False] = False, split: None) -> str:
    ...


@overload
def load(path: Path | str, *, parseint: Literal[False] = False, split: str = ...) -> list[str]:
    ...


def load(
    path: Path | str, *, parseint: bool = False, split: Optional[str] = "\n"
) -> str | Sequence[str] | list[int]:
    if not Path(path).is_absolute():
        path = Path(__file__).parent.parent.parent / "data" / "2020" / path

    out = Path(path).read_text().strip()
    out = out.split(split) if split is not None else out

    if parseint:
        return [int(x) for x in out]

    return out
