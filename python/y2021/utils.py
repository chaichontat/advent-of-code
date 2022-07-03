from pathlib import Path
from typing import Callable, Literal, TypeVar, overload

T = TypeVar("T")


@overload
def read(path: str) -> list[str]:
    ...


@overload
def read(path: str, *, raw: Literal[True]) -> str:
    ...


@overload
def read(path: str, *, parse: Callable[[str], T]) -> list[T]:
    ...


def read(path, raw: bool = False, parse: Callable[[str], T] | None = None) -> list[T] | list[str] | str:
    lines = (r := Path(path).read_text()).splitlines()
    if raw:
        return r
    if parse:
        return list(map(parse, lines))
    return lines
