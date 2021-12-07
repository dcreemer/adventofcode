# tests for day seven

from ..seven import parse_input, part_one_calc, part_two_calc

input = """16,1,2,0,4,2,7,1,2,14
"""


def test_parse_input() -> None:
    data = parse_input(input)
    assert data == [16, 1, 2, 0, 4, 2, 7, 1, 2, 14]


def test_part_one() -> None:
    data = parse_input(input)
    assert part_one_calc(data) == 37


def test_part_two() -> None:
    data = parse_input(input)
    assert part_two_calc(data) == 168
