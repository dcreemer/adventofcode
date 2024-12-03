# tests for day02 functions

from aoc2024 import day03, parse

sample_input = """
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"""


def test_part_1() -> None:
    result = day03.part_1(parse.strip_lines(sample_input))
    assert result == 0


def test_part_2() -> None:
    result = day03.part_2(parse.strip_lines(sample_input))
    assert result == 0


def test_version() -> None:
    assert day03.DAY == "03"
