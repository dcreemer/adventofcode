# tests for day01 functions

from aoc2024 import day01, parse

sample_input = """
3   4
4   3
2   5
1   3
3   9
3   3
"""


def test_pair_of_lists() -> None:
    lines = parse.strip_lines(sample_input)
    list_1, list_2 = day01.pair_of_lists(lines)
    assert list_1 == [3, 4, 2, 1, 3, 3]
    assert list_2 == [4, 3, 5, 3, 9, 3]


def test_part_1() -> None:
    result = day01.part_1(parse.strip_lines(sample_input))
    assert result == 11


def test_part_2() -> None:
    result = day01.part_2(parse.strip_lines(sample_input))
    assert result == 31


def test_version() -> None:
    assert day01.DAY == "01"
