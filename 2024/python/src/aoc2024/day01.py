# AoC 2024 Day 01 Solution

from collections import Counter

from lib import files
from lib.parse import StringList

DAY = "01"
INPUT = files.read_to_lines(files.input_file(DAY))


def pair_of_lists(lines: StringList) -> tuple[list[int], list[int]]:
    """
    Parse the input lines into two lists of integers.
    """
    list_1: list[int] = []
    list_2: list[int] = []
    for a, b in [line.split("   ") for line in lines]:
        list_1.append(int(a))
        list_2.append(int(b))
    return (list_1, list_2)


def part_1(lines: StringList) -> int:
    list_1, list_2 = pair_of_lists(lines)
    # calculate "distance" between lists by
    # summing up absolute differences between corresponding elements
    # in sorted lists.
    return sum([abs(a - b) for a, b in zip(sorted(list_1), sorted(list_2), strict=True)])


def part_2(lines: StringList) -> int:
    list_1, list_2 = pair_of_lists(lines)
    # calculate frequence of numbers in list_2:
    frequencies: Counter[int] = Counter(list_2)
    # calculate "list similarity" by
    # multiplying each number in list_1 by its frequency in list_2
    # and summing up all these products.
    return sum(frequencies[n] * n for n in list_1)


def main() -> None:
    print("Advent of Code Day", DAY)
    print("Part 1 Result: ", part_1(INPUT))
    print("Part 2 Result: ", part_2(INPUT))
