# tests for day one

from ..one import parse_input, part_one_calc, part_two_calc, sliding_window

input = """199
200
208
210
200
207
240
260
269
263"""


def test_parse_input() -> None:
    x = parse_input(input)
    assert len(x) == 10


def test_sliding_window() -> None:
    assert ["".join(x) for x in sliding_window("ABCDEFG", 4)] == ["ABCD", "BCDE", "CDEF", "DEFG"]


def test_part_one() -> None:
    depths = parse_input(input)
    assert part_one_calc(depths) == 7


def test_part_two() -> None:
    depths = parse_input(input)
    assert part_two_calc(depths) == 5
