# tests for day one

from ..main import (
    calculate_calibration_sum_one,
    calculate_calibration_sum_two,
    parse_calibration_data,
    parse_line_to_digits,
)

input1 = """1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"""

input2 = """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"""


def test_parse_input():
    expected = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
    assert parse_calibration_data(input1) == expected


def test_one():
    assert calculate_calibration_sum_one(parse_calibration_data(input1)) == 142


def test_two():
    assert calculate_calibration_sum_two(parse_calibration_data(input2)) == 281


def test_parse_line_to_digits():
    assert parse_line_to_digits("two1nine") == [2, 1, 9]
    assert parse_line_to_digits("eightwothree") == [8, 2, 3]
    assert parse_line_to_digits("abcone2threexyz") == [1, 2, 3]
    assert parse_line_to_digits("xtwone3four") == [2, 1, 3, 4]
    assert parse_line_to_digits("4nineeightseven2") == [4, 9, 8, 7, 2]
    assert parse_line_to_digits("zoneight234") == [1, 8, 2, 3, 4]
    assert parse_line_to_digits("7pqrstsixteen") == [7, 6]
