from pathlib import Path


def load(path: str, parseint=False, split="\n") -> list:
    raw = Path(path).read_text().split(split)
    if parseint:
        return [int(x) for x in raw]
    return raw
