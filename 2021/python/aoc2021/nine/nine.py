# AoC 2021 day nine

from functools import reduce
from operator import mul
from typing import Tuple

from ..lib.input import read_raw_input_data


def parse_input(input: str) -> list[list[int]]:
    return [[int(c) for c in line] for line in input.splitlines() if line]


def neighbors(x: int, y: int, data: list[list[int]]) -> list[Tuple[int, int]]:
    result = []
    if x > 0:
        result.append((x-1, y))
    if x < len(data[y])-1:
        result.append((x+1, y))
    if y > 0:
        result.append((x, y-1))
    if y < len(data)-1:
        result.append((x, y+1))
    return result


def low_points(data: list[list[int]]) -> list[Tuple[int, int]]:
    low_points = []
    for y in range(len(data)):
        for x in range(len(data[y])):
            if data[y][x] < min(data[ny][nx] for (nx, ny) in neighbors(x, y, data)):
                low_points.append((x, y))
    return low_points


def part_one_calc(data: list[list[int]]) -> int:
    low_point_values = low_points(data)
    return sum([data[y][x] + 1 for (x, y) in low_point_values])


def basin_size(data: list[list[int]], x: int, y: int) -> int:
    visited = set()
    to_visit = [(x, y)]
    while to_visit:
        x, y = to_visit.pop()
        if (x, y) in visited:
            continue
        visited.add((x, y))
        for (nx, ny) in neighbors(x, y, data):
            if data[ny][nx] < 9:
                to_visit.append((nx, ny))
    return len(visited)


def part_two_calc(data: list[list[int]]) -> int:
    lps = low_points(data)
    return reduce(mul, sorted([basin_size(data, x, y) for (x, y) in lps])[-3:], 1)


def main() -> None:
    print("Day Nine")
    data = read_raw_input_data(__file__)
    stuff = parse_input(data)
    print("Part One:", part_one_calc(stuff))
    print("Part Two:", part_two_calc(stuff))


if __name__ == "__main__":
    main()
