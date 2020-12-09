from pathlib import Path


def load(path: str, **kwargs) -> list:
    return _load(Path(path).read_text(), **kwargs)


def _load(raw, parseint=False, split="\n"):
    raw = raw.split(split)
    if not raw[-1]:
        raw = raw[:-1]
    if parseint:
        return [int(x) for x in raw]
    return raw