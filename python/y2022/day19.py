# %%
import operator as op
import re
from concurrent.futures import ProcessPoolExecutor, wait
from copy import copy
from dataclasses import dataclass
from enum import Enum
from functools import reduce
from pathlib import Path
from typing import Annotated, Self

import seaborn as sns

sns.set()
raw = (
    Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt")
    .read_text()[:-1]
    .split("\n")
)

# raw = """Blueprint 2:
#   Each ore robot costs 2 ore.
#   Each clay robot costs 3 ore.
#   Each obsidian robot costs 3 ore and 8 clay.
#   Each geode robot costs 3 ore and 12 obsidian.""".replace(
#     "\n", ""
# )
Ore = Annotated[int, "ore"]
Clay = Annotated[int, "clay"]


@dataclass
class Spec:
    id: int
    ore_robot: Ore
    clay_robot: Ore
    obsidian_robot: tuple[Ore, Clay]
    geode_robot: tuple[Ore, Annotated[int, "obsidian"]]

    @classmethod
    def from_raw(cls, raw: str) -> Self:
        res = re.findall(r"(\d+)", raw)
        return cls(
            id=int(res[0]),
            ore_robot=int(res[1]),
            clay_robot=int(res[2]),
            obsidian_robot=(int(res[3]), int(res[4])),
            geode_robot=(int(res[5]), int(res[6])),
        )


Actions = Enum("Actions", "BuildOre BuildClay BuildObsidian BuildGeode")


@dataclass
class Inventory:
    spec: Spec
    ore_robot: int = 1
    clay_robot: int = 0
    obsidian_robot: int = 0
    geode_robot: int = 0
    ore: int = 0
    clay: int = 0
    obsidian: int = 0
    geode: int = 0
    limit: int = 24


def step(inventory: Inventory, t: int, actions: list[Actions | None]):
    if t >= inventory.limit:
        return inventory.geode

    spec = inventory.spec
    inventory.ore += inventory.ore_robot
    inventory.clay += inventory.clay_robot
    inventory.obsidian += inventory.obsidian_robot
    inventory.geode += inventory.geode_robot

    if actions:
        match actions[-1]:
            case Actions.BuildOre:
                inventory.ore_robot += 1
            case Actions.BuildClay:
                inventory.clay_robot += 1
            case Actions.BuildObsidian:
                inventory.obsidian_robot += 1
            case Actions.BuildGeode:
                inventory.geode_robot += 1

    if (
        inventory.ore >= spec.geode_robot[0]
        and inventory.obsidian >= spec.geode_robot[1]
    ):
        new_inventory = copy(inventory)
        new_inventory.ore -= spec.geode_robot[0]
        new_inventory.obsidian -= spec.geode_robot[1]
        # print(new_inventory.geode_robot)
        return step(new_inventory, t + 1, actions + [Actions.BuildGeode])

    res: list[int] = []

    if (
        inventory.ore >= spec.obsidian_robot[0]
        and inventory.clay >= spec.obsidian_robot[1]
    ):
        new_inventory = copy(inventory)
        new_inventory.ore -= spec.obsidian_robot[0]
        new_inventory.clay -= spec.obsidian_robot[1]
        r = step(new_inventory, t + 1, actions + [Actions.BuildObsidian])
        if inventory.obsidian <= spec.geode_robot[1]:
            return r  # if there's no obsidian, always make sense to make robot

    if (
        inventory.ore >= spec.ore_robot
        and inventory.ore < max(spec.obsidian_robot[0], spec.geode_robot[0]) + 2
    ):
        new_inventory = copy(inventory)
        new_inventory.ore -= spec.ore_robot
        res.append(step(new_inventory, t + 1, actions + [Actions.BuildOre]))

    if (
        inventory.ore >= spec.clay_robot
        and inventory.ore < max(spec.obsidian_robot[1], spec.geode_robot[1]) + 2
    ):
        new_inventory = copy(inventory)
        new_inventory.ore -= spec.clay_robot
        res.append(step(new_inventory, t + 1, actions + [Actions.BuildClay]))

    res.append(step(inventory, t + 1, actions + [None]))
    return max(res)


# %%


def part1(raw: str, limit: int) -> int:
    spec = Spec.from_raw(raw)
    return spec.id * step(Inventory(spec, limit=limit), 0, [])


def part2(raw: str, limit: int) -> int:
    spec = Spec.from_raw(raw)
    return step(Inventory(spec, limit=limit), 0, [])


if __name__ == "__main__":
    with ProcessPoolExecutor(8) as executor:
        futs1 = [executor.submit(part1, r, 24) for r in raw]
        futs2 = [executor.submit(part2, r, 32) for r in raw[:3]]

        done, _ = wait(futs1)
        print("Part 1:", sum(f.result() for f in done))
        done, _ = wait(futs2)
        print("Part 2:", reduce(op.mul, (f.result() for f in done)))
