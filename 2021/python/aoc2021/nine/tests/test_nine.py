# tests for day nine

from ..nine import parse_input, part_one_calc, part_two_calc

input = """
2199943210
3987894921
9856789892
8767896789
9899965678
"""


def test_parse_input() -> None:
    data = parse_input(input)
    assert len(data) == 5
    assert data[0] == [2, 1, 9, 9, 9, 4, 3, 2, 1, 0]


def test_part_one() -> None:
    data = parse_input(input)
    assert part_one_calc(data) == 15


def test_part_two() -> None:
    data = parse_input(input)
    assert part_two_calc(data) == 1134
