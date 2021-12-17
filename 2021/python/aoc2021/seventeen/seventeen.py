# AoC 2021 day Seventeen

import re
import sys
from dataclasses import dataclass
from typing import Optional, Tuple

from ..lib.input import read_raw_input_data


@dataclass
class Target:
    min_x: int
    max_x: int
    min_y: int
    max_y: int


def parse_input(input: str) -> Target:
    m = re.search(
        r"^target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)$",
        input.strip()
    )
    if m is None:
        raise ValueError("Invalid input")
    return Target(
        int(m.group(1)),
        int(m.group(2)),
        int(m.group(3)),
        int(m.group(4))
    )


def step(x: int, y: int, xv: int, yv: int) -> Tuple[int, int, int, int]:
    x += xv
    y += yv
    if xv > 0:
        xv -= 1
    elif xv < 0:
        xv += 1
    yv -= 1
    return x, y, xv, yv


def in_target(x: int, y: int, target: Target) -> bool:
    return \
        x >= target.min_x and \
        x <= target.max_x and \
        y >= target.min_y and \
        y <= target.max_y


def probe_one(xv: int, yv: int, target: Target) -> Optional[int]:
    x, y = 0, 0
    max_y = y
    while not in_target(x, y, target):
        x, y, xv, yv = step(x, y, xv, yv)
        if y > max_y:
            max_y = y
        # are we past the point of never succeeding?
        if y < target.min_y and yv <= 0:
            return None
        if xv <= 0 and x < target.min_x:
            return None
        if x > target.max_x and xv >= 0:
            return None
    return max_y


def probe(target: Target) -> Tuple[int, set[Tuple[int, int]]]:
    peak = -sys.maxsize
    hits = set()
    for xv in range(1, target.max_x+1):
        for yv in range(target.min_y, -target.min_y):
            max_y = probe_one(xv, yv, target)
            if max_y is not None:
                hits.add((xv, yv))
                if max_y > peak:
                    peak = max_y
    return peak, hits


def part_one_calc(target: Target) -> int:
    peak, _ = probe(target)
    return peak


def part_two_calc(target: Target) -> int:
    _, hits = probe(target)
    return len(hits)


def main() -> None:
    print("Day Seventeen")
    data = read_raw_input_data(__file__)
    stuff = parse_input(data)
    print("Target:", stuff)
    print("Part One:", part_one_calc(stuff))
    print("Part Two:", part_two_calc(stuff))


if __name__ == "__main__":
    main()
