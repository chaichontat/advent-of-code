#%%
from utils import load

raw = load("day01.txt", parseint=True)

#%% Part 1
def calc_fuel_from_mass(mass):
    """
    Fuel required to launch a given module is based on its mass.
    Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.
    """
    return int(mass / 3) - 2


def test01a(benchmark):
    assert benchmark(lambda: sum([calc_fuel_from_mass(x) for x in raw])) == 3184233


#%% Part 2
def calc_fuel_from_fuel(fuel):
    return calc_fuel_from_fuel(x) + x if (x := calc_fuel_from_mass(fuel)) > 0 else 0


def test01b(benchmark):
    assert benchmark(lambda: sum([calc_fuel_from_fuel(x) for x in raw])) == 4773483


# %%
