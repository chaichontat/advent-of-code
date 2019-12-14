import numpy as np


def calc_fuel_from_mass(mass):
    return np.floor(mass / 3) - 2


def calc_fuel_from_fuel(fuel):
    if (x := calc_fuel_from_mass(fuel)) > 0:
        return calc_fuel_from_fuel(x) + x
    else:
        return 0


if __name__ == '__main__':
    examples = {
        12: 2,
        14: 2,
        1969: 654,
        100756: 33583
    }

    for m, f in examples.items():
        assert calc_fuel_from_mass(m) == f

    modules = np.loadtxt('input_day1.txt')

    # Part 1
    print(np.sum(calc_fuel_from_mass(modules)))

    # Part 2
    print(np.sum([calc_fuel_from_fuel(module) for module in modules]))
