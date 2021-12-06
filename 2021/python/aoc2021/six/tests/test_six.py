# tests for day six

from ..six import parse_input, part_one_calc, part_two_calc

input = """3,4,3,1,2
"""


def test_parse_input() -> None:
    fish_counts = parse_input(input)
    assert fish_counts == [0, 1, 1, 2, 1, 0, 0, 0, 0]


def test_part_one() -> None:
    fish = parse_input(input)
    assert part_one_calc(fish) == 5934


def test_part_two() -> None:
    fish = parse_input(input)
    assert part_two_calc(fish) == 26984457539
