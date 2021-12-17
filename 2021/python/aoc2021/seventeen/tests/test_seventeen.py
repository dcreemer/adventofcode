# tests for day Seventeen

from ..seventeen import parse_input, part_one_calc, part_two_calc

input = """
target area: x=20..30, y=-10..-5
"""


def test_parse_input() -> None:
    data = parse_input(input)
    assert data.min_x == 20
    assert data.max_x == 30
    assert data.min_y == -10
    assert data.max_y == -5


def test_part_one() -> None:
    data = parse_input(input)
    assert part_one_calc(data) == 45


def test_part_two() -> None:
    data = parse_input(input)
    assert part_two_calc(data) == 112
