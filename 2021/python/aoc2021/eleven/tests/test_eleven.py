# tests for day eleven

from ..eleven import parse_input, part_one_calc, part_two_calc

input = """
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"""


def test_parse_input() -> None:
    data = parse_input(input)
    assert len(data) == 10
    assert data[0][0] == 5
    assert data[9][9] == 6


def test_part_one() -> None:
    data = parse_input(input)
    assert part_one_calc(data) == 1656


def test_part_two() -> None:
    data = parse_input(input)
    assert part_two_calc(data) == 195
