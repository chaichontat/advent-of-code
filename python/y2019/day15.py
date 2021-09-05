#%%
from intcode import IntCode
from utils import load

raw = load("day15.txt", split=",", parseint=True)

#%%
class Repair(IntCode):
    def __init__(self, instructions: list[int]) -> None:
        super().__init__(instructions)
        assert self.execute() == 1

    def step(self, input_=None):
        assert input_ in [1, 2, 3, 4]
        self.inputs.append(input_)
        out = self.execute()
        assert out == 1
        return self.outputs.pop()


move = {
    1j: 1,
    -1j: 2,
    -1: 3,
    1: 4,
}


def explore(ic: Repair, ori=0 + 0j):
    """Un-informed cost search. Must use DFS since we're moving our bot."""

    mapp = dict()
    target = 0

    def dfs(curr_pos, hdg, curr_dist):
        nonlocal target
        new_pos = curr_pos + hdg
        if new_pos in mapp:
            return

        found = mapp[new_pos] = ic.step(move[hdg])
        if found == 0:  # Wall
            return

        if found == 2:
            target = new_pos

        for new_hdg in move:
            dfs(new_pos, new_hdg, curr_dist + 1)

        ic.step(move[-hdg])  # Go back.

    for new_hdg in move:
        dfs(ori, new_hdg, 0)

    return target, mapp


ic = Repair(raw)
target, mapp = explore(ic)


def find_farthest(source, mapp, target=None):
    """
    BFS.
    Find shortest distance from source to target if given target.
    Else find the longest distance possible.
    """

    points_from_dist = {0: (source,)}
    curr_dist = 1
    checked = set([source])

    while points_from_dist[curr_dist - 1]:
        next_points = list()
        for point in points_from_dist[curr_dist - 1]:
            for new_hdg in move:
                new_point = point + new_hdg
                if target == new_point:
                    return curr_dist

                if new_point not in checked and mapp.get(new_point, 0) == 1:
                    next_points.append(point + new_hdg)
                    checked.add(new_point)

        points_from_dist[curr_dist] = tuple(next_points)
        curr_dist += 1

    return curr_dist - 2


def test1() -> None:
    assert find_farthest(0 + 0j, mapp, target) == 216


def test2() -> None:
    assert find_farthest(target, mapp) == 326
