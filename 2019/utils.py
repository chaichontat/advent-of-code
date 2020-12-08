from pathlib import Path


def load(path: str, parseint=False, split="\n") -> list:
    raw = Path(path).read_text().split(split)
    if not raw[-1]:
        raw = raw[:-1]
    if parseint:
        return [int(x) for x in raw]
    return raw
