# AoC 2024 Day 01 Solution

import itertools
from collections import defaultdict

from lib import files

DAY = "01"
INPUT = files.read_to_lines(files.input_file(DAY))


def pair_of_lists(lines: list[str]) -> tuple[list[int], list[int]]:
    """
    Parse the input lines into two lists of integers.
    """
    list_1: list[int] = []
    list_2: list[int] = []
    for line in lines:
        a, b = line.split("   ")
        list_1.append(int(a))
        list_2.append(int(b))
    return (list_1, list_2)


def part_1(lines: list[str]) -> int:
    list_1, list_2 = pair_of_lists(lines)
    # calculate "distance" between lists by
    # summing up absolute differences between corresponding elements
    # in sorted lists.
    result = 0
    for a, b in itertools.zip_longest(sorted(list_1), sorted(list_2)):
        result += abs(a - b)
    return result


def part_2(lines: list[str]) -> int:
    list_1, list_2 = pair_of_lists(lines)
    # calculate frequence of numbers in list_2:
    frequencies: dict[int, int] = defaultdict(int)
    for n in list_2:
        frequencies[n] += 1
    # calculate "list similarity" by
    # multiplying each number in list_1 by its frequency in list_2
    # and summing up all these products.
    result = 0
    for n in list_1:
        f = frequencies.get(n, 0)
        result += n * f
    return result


def main() -> None:
    print("Advent of Code Day", DAY)
    print("Part 1 Result: ", part_1(INPUT))
    print("Part 2 Result: ", part_2(INPUT))
