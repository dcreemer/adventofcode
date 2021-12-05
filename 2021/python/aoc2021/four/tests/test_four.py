# tests for day two

from ..four import Coord, parse_input, part_one_calc, part_two_calc

input = """7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"""


def test_parse_input() -> None:
    nums, cards = parse_input(input)
    assert len(nums) == 27
    assert nums[0] == 7
    assert nums[-1] == 1
    assert len(cards) == 3
    assert cards[0].nums[22] == Coord(0, 0)


def test_card_fn() -> None:
    nums, cards = parse_input(input)
    assert not cards[0].check_win()
    cards[0].mark_called(22)
    cards[0].mark_called(8)
    cards[0].mark_called(21)
    cards[0].mark_called(6)
    assert not cards[0].check_win()
    cards[0].mark_called(1)
    assert cards[0].check_win()


def test_part_one() -> None:
    nums, cards = parse_input(input)
    part_one_calc(nums, cards) == 4512


def test_part_two() -> None:
    nums, cards = parse_input(input)
    part_one_calc(nums, cards) == 1924
