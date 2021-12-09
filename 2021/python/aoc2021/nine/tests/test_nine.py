# tests for day nine

from ..nine import parse_input, part_one_calc, part_two_calc

input = """
"""


def test_parse_input() -> None:
    data = parse_input(input)
    assert len(data) == 0


def test_part_one() -> None:
    data = parse_input(input)
    assert part_one_calc(data) == 1


def test_part_two() -> None:
    data = parse_input(input)
    assert part_two_calc(data) == 2
