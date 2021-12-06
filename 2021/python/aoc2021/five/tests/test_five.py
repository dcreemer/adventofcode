# tests for day two

from ..five import parse_input, part_one_calc, part_two_calc

input = """0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"""


def test_parse_input() -> None:
    lines = parse_input(input)
    assert len(lines) == 10


def test_part_one() -> None:
    lines = parse_input(input)
    assert part_one_calc(lines) == 5


def test_part_two() -> None:
    lines = parse_input(input)
    assert part_two_calc(lines) == 12
