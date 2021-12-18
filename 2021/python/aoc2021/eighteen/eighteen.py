# AoC 2021 day Eighteen

# I struggled with how to implement explode on this one, and still don't like it.
# this splode structure is adapted from a hint on Reddit. What would be nice is
# to find a "regex"-like language for finding patterns in these trees, and using
# that to express what I'm looking for and what I want to modify.

import json
import math
from functools import reduce
from itertools import permutations
from typing import Optional, Tuple, Union

from ..lib.input import read_raw_input_data

# I would really like to represent numbers as a recursively defined type,
# but that isn't supported yet by mypy. Something Like:
# Number = Union[Tuple[Number, Number], int]
# Currently there's nothing to enforce that the list is of length 2.
Number = Union[list, int]


def parse_input(data: str) -> list[Number]:
    return [json.loads(line) for line in data.splitlines() if line]


def add_right(n: Number, val: Optional[Number]) -> Number:
    if val is None:
        # we've reached the edge
        return n
    elif isinstance(n, int):
        if isinstance(val, int):
            # add two literals
            return n + val
        else:
            raise ValueError("Can't add a composite to a literal", n, val)
    else:
        # keep moving right
        return [n[0], add_right(n[1], val)]


def add_left(n: Number, val: Optional[Number]) -> Number:
    if val is None:
        # we've reached the edge
        return n
    if isinstance(n, int):
        if isinstance(val, int):
            # add two literals
            return n + val
        else:
            raise ValueError("Can't add a composite to a literal", n, val)
    else:
        # keep moving left
        return [add_left(n[0], val), n[1]]


def explode_inner(num: Number, ctr: int) -> Tuple[Optional[Number], Number, Optional[Number], bool]:
    # returns (keep_going, new_left, new_list, new_right)
    if isinstance(num, int):
        # we've reached a literal. Stop and return it.
        return None, num, None, False
    if ctr == 0:
        # we've reached a pair to explode. Replace the pair with a 0,
        # and then add-left and add-right the left and right values:
        return num[0], 0, num[1], True
    # we're not at a literal (so we're at a composite number)
    # so recur down the left side until a condition above hits ...
    lhs, rhs = num
    left, lhs, right, keep_going = explode_inner(lhs, ctr - 1)
    if keep_going:
        # we found a suitable pair to explode on the left side,
        # add the number to the left
        return left, [lhs, add_left(rhs, right)], None, True
    # we didn't find a suitable pair to explode on the left side,
    # so recur down the right side until a condition above hits ...
    left, rhs, right, keep_going = explode_inner(rhs, ctr - 1)
    if keep_going:
        # we found a suitable pair to explode on the right side,
        # add the number to the right
        return None, [add_right(lhs, left), rhs], right, True
    # we didn't find a suitable pair to explode, so stop and return
    # the composite number
    return None, num, None, False


def explode(num: Number) -> Tuple[Number, bool]:
    _, new_num, _, changed = explode_inner(num, 4)
    return new_num, changed


def split(n: Number) -> Tuple[Number, bool]:
    # "split" a number, by searching depth-first down the "left"
    # side looking for a literal rather than composite number.
    # Returning true if we found and changed a literal.
    # proceed left first looking for a number to split
    if isinstance(n, list):
        # composite:
        new_left, changed = split(n[0])
        if changed:
            return [new_left, n[1]], True
        new_right, changed = split(n[1])
        return [n[0], new_right], changed
    else:
        # literal:
        if n >= 10:
            return [n // 2, math.ceil(n / 2)], True
        else:
            return n, False


def add(lhs: Number, rhs: Number) -> Number:
    num: Number = [lhs, rhs]
    # reduce the number according to the Day 18 rules
    changed = True
    while changed:
        num, changed = explode(num)
        if changed:
            continue
        num, changed = split(num)
    return num


def magnitude(e: Number) -> int:
    if isinstance(e, list):
        return 3 * magnitude(e[0]) + 2 * magnitude(e[1])
    else:
        return e


def part_one_calc(data: list[Number]) -> int:
    init: Number = 0
    return magnitude(reduce(add, data, init))


def part_two_calc(data: list[Number]) -> int:
    # find the maximum ...
    return max(
        # ... of the magnitudes ...
        [magnitude(n) for n in [
            # of the sums of every pair of numbers
            add(p[0], p[1]) for p in permutations(data, 2)
        ]])


def main() -> None:
    print("Day Eighteen")
    data = read_raw_input_data(__file__)
    stuff = parse_input(data)
    print("Part One:", part_one_calc(stuff))
    print("Part Two:", part_two_calc(stuff))


if __name__ == "__main__":
    main()
