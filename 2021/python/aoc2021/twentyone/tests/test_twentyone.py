# tests for day Twenty-one

from ..twentyone import parse_input, part_one_calc, part_two_calc

input = """
Player 1 starting position: 4
Player 2 starting position: 8
"""


def test_parse_input() -> None:
    a, b = parse_input(input)
    assert (a, b) == (4, 8)


def test_part_one() -> None:
    a, b = parse_input(input)
    assert part_one_calc(a, b) == 739785


def test_part_two() -> None:
    a, b = parse_input(input)
    assert part_two_calc(a, b) == 444356092776315
