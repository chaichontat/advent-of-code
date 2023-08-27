from functools import wraps
from typing import Callable, Generic, Iterable, Iterator, ParamSpec, TypeVar

P, R = ParamSpec("P"), TypeVar("R")
T = TypeVar("T")


def fmap(f: Callable[[T], R], container: Iterable[T]) -> Iterable[R]:
    return container.__class__(map(f, container))
