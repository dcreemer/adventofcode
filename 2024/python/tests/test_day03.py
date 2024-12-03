# tests for day02 functions

from aoc2024 import day03, parse

sample_input = """
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"""

sample_input_2 = """
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"""


def test_parse_input() -> None:
    v = day03.parse_input(sample_input)
    assert v == [day03.Multiply(2, 4), day03.Multiply(5, 5), day03.Multiply(11, 8), day03.Multiply(8, 5)]
    v = day03.parse_input(sample_input_2)
    assert v == [
        day03.Multiply(2, 4),
        day03.Dont(),
        day03.Multiply(5, 5),
        day03.Multiply(11, 8),
        day03.Do(),
        day03.Multiply(8, 5),
    ]


def test_part_1() -> None:
    result = day03.part_1(parse.strip_lines(sample_input))
    assert result == 161


def test_part_2() -> None:
    result = day03.part_2(parse.strip_lines(sample_input_2))
    assert result == 48


def test_version() -> None:
    assert day03.DAY == "03"
