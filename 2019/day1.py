#%%
from utils import load

raw = load("input_day1.txt", parseint=True)

#%% Part 1


def calc_fuel_from_mass(mass):
    """
    Fuel required to launch a given module is based on its mass.
    Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.
    """
    return int(mass / 3) - 2


print(sum([calc_fuel_from_mass(x) for x in raw]))

#%% Part 2


def calc_fuel_from_fuel(fuel):
    if (x := calc_fuel_from_mass(fuel)) > 0:
        return calc_fuel_from_fuel(x) + x
    else:
        return 0


print(sum([calc_fuel_from_fuel(x) for x in raw]))

# %%
