# tests for day Twelve

from ..twelve import parse_input, part_one_calc, part_two_calc

input = """
start-A
start-b
A-c
A-b
b-d
A-end
b-end
"""


def test_parse_input() -> None:
    data = parse_input(input)
    assert len(data) == 6


def test_part_one() -> None:
    data = parse_input(input)
    assert part_one_calc(data) == 10


def test_part_two() -> None:
    data = parse_input(input)
    assert part_two_calc(data) == 36
