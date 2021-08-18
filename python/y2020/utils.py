from pathlib import Path


def load(path: str, **kwargs) -> list:
    if not Path(path).is_absolute():
        path = Path(__file__).parent.parent.parent / "data" / "2020" / path
    return _load(Path(path).read_text(), **kwargs)


def _load(raw, parseint=False, split="\n"):
    raw = raw.strip()
    if split is not None:
        raw = raw.split(split)
    if parseint:
        return [int(x) for x in raw]
    return raw
