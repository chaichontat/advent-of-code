import collections
from itertools import islice
from typing import Callable, Iterable, Iterator, ParamSpec, TypeVar

P, R = ParamSpec("P"), TypeVar("R")
T = TypeVar("T")


def fmap(
    f: Callable[[T], R], container: Iterable[T] | None = None
) -> Iterable[R] | Callable[[Iterable[T]], Iterable[R]]:
    def _inner(container: Iterable[T]) -> Iterable[R]:
        return fmap(f, container)

    if container is None:
        return _inner
    return container.__class__(map(f, container))


def sliding_window(iterable: Iterable[T], n: int) -> Iterator[tuple[T, ...]]:
    # sliding_window('ABCDEFG', 4) --> ABCD BCDE CDEF DEFG
    it = iter(iterable)
    window = collections.deque(islice(it, n), maxlen=n)
    if len(window) == n:
        yield tuple(window)
    for x in it:
        window.append(x)
        yield tuple(window)
