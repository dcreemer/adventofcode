# tests for day XYZ

from ..thirteen import parse_input, part_one_calc, part_two_calc

input = """
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"""


def test_parse_input() -> None:
    dots, folds = parse_input(input)
    assert len(dots) == 18
    assert len(folds) == 2


def test_part_one() -> None:
    dots, folds = parse_input(input)
    assert part_one_calc(dots, folds[0]) == 17


def test_part_two() -> None:
    dots, folds = parse_input(input)
    assert part_two_calc(dots, folds) == """
#####
#...#
#...#
#...#
#####
.....
.....
""".strip()
