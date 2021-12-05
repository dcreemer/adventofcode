# tests for day two

from ..three import bit_filter, lcb, mcb, parse_input, part_one_calc, part_two_calc

input = """00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"""


def test_parse_input() -> None:
    x = parse_input(input)
    assert len(x) == 12


def test_part_one() -> None:
    nums = parse_input(input)
    part_one_calc(nums) == 198


def test_part_two() -> None:
    nums = parse_input(input)
    oxy = bit_filter(mcb, nums)
    assert oxy == 23
    co2 = bit_filter(lcb, nums)
    assert co2 == 10
    assert part_two_calc(nums) == 230
