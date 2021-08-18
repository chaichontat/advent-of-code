#%%
from utils import load

raw = load("day13.txt")

# %%
def test1():
    t = int(raw[0])
    buses = [int(x) for x in raw[1].split(",") if x != "x"]

    mod = [bus - (t % bus) for bus in buses]
    tmin = min(mod)
    tbus = buses[mod.index(tmin)]
    assert tmin * tbus == 2845


#%%
# https://bugs.python.org/issue39657
def bezout(a: int, b: int) -> tuple[int, int, int]:
    """
    Extended Euclidean Algorithm.
    Given integers a and b, return a tuple (x, y, g),
    where x*a + y*b == g == gcd(a, b).
    """

    u1, v1, r1 = 1, 0, a
    u2, v2, r2 = 0, 1, b

    while r2:
        q = r1 // r2
        u1, u2 = u2, u1 - q * u2
        v1, v2 = v2, v1 - q * v2
        r1, r2 = r2, r1 - q * r2
        assert u1 * a + v1 * b == r1
        assert u2 * a + v2 * b == r2

    if r1 < 0:
        u1, v1, r1 = -u1, -v1, -r1

    return (u1, v1, r1)


def crt(*congs: list[tuple[int, int]]) -> tuple[int, int]:
    """
    Chinese Remainder Theorem. Iteratively solve pair-wise.
    """
    congs = list(congs)

    def pair(cong1, cong2):
        a1, b1 = cong1
        a2, b2 = cong2
        m1, m2, g = bezout(b1, b2)

        if (a1 - a2) % g != 0:
            raise ValueError(f"No solution {cong1} and {cong2}.")

        lcm = b1 // g * b2
        rem = (a1 * m2 * b2 + a2 * m1 * b1) // g
        return rem % lcm, lcm

    while len(congs) > 1:
        x, y = congs.pop(0), congs.pop(0)
        congs.append(pair(x, y))

    return congs[0]


# %%
def test2():
    buses = [(int(x) - i, int(x)) for i, x in enumerate(raw[1].split(",")) if x != "x"]
    assert crt(*buses)[0] == 487905974205117


# %%
