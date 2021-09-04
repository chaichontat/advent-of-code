from __future__ import annotations  # Until Python 3.10

from pathlib import Path
from typing import Literal, Optional, overload


@overload
def load(path: Path | str, *, parseint: Literal[True], split: Optional[str] = ...) -> list[int]:
    ...


@overload
def load(path: Path | str, *, parseint: bool = False, split: None) -> str:
    ...


@overload
def load(path: Path | str, *, parseint: bool = False, split: str = ...) -> list[str]:
    ...


def load(path: Path | str, *, parseint: bool = False, split: Optional[str] = "\n"):
    if not Path(path).is_absolute():
        path = Path(__file__).parent.parent.parent / "data" / "2020" / path

    out = Path(path).read_text().strip()
    if split is not None:
        out = out.split(split)
    if parseint:
        return [int(x) for x in out]
    return out
