#%%
from math import ceil

from utils import load

raw = load("input_day14.txt")

# %%


def process(raw):
    materials = dict()
    prod_amount = dict()

    def parse(line):
        prod_amount["ORE"] = 1
        materials["ORE"] = {"ORE": 1}

        reactants, product = line.split(" => ")
        n_prod, product = product.split(" ")
        prod_amount[product] = int(n_prod)

        reactants = reactants.split(", ")
        materials[product] = dict()
        for reactant in reactants:
            n, r = reactant.split(" ")
            materials[product][r] = int(n)

    [parse(x) for x in raw]
    return materials, prod_amount


def append(dic, k, v):
    if k in dic:
        dic[k] += v
    else:
        dic[k] = v


materials, prod_amount = process(raw)


def ore_from_fuel(n_fuel=1):
    required = {"FUEL": n_fuel}
    produced = {}
    ore_needed = 0

    while required:
        tg, tg_needed = list(required.items())[0]
        if tg == "ORE":
            ore_needed += required.pop("ORE")
            continue

        if (tg_to_make := tg_needed - produced.get(tg, 0)) > 0:  # Need to make more.
            rt_to_run = ceil(tg_to_make / prod_amount[tg])
            append(produced, tg, prod_amount[tg] * rt_to_run)
            [append(required, k, rt_to_run * v) for k, v in materials[tg].items()]

        produced[tg] -= tg_needed
        assert produced[tg] >= 0
        del required[tg]
    return ore_needed


def test1():
    assert ore_from_fuel(1) == 892207


# %%


def test2():
    lo = 1e12 // ore_from_fuel(1)
    hi = 5 * lo
    mid = (lo + hi) // 2

    while hi - lo > 1:
        if ore_from_fuel(mid) > 1e12:
            hi = mid
        else:
            lo = mid
        mid = (lo + hi) // 2

    assert lo == 1935265


# %%
