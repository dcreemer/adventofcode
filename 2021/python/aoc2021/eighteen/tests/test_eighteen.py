# tests for day Eighteen

from ..eighteen import (
    add,
    explode,
    magnitude,
    parse_input,
    part_one_calc,
    part_two_calc,
    split
)

input = """
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
"""

input2 = """
[[1,2],[[3,4],5]]
[[[[0,7],4],[[7,8],[6,0]]],[8,1]]
[[[[1,1],[2,2]],[3,3]],[4,4]]
[[[[3,0],[5,3]],[4,4]],[5,5]]
[[[[5,0],[7,4]],[5,5]],[6,6]]
[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]
"""


def test_parse_input() -> None:
    data = parse_input(input)
    assert len(data) == 10


def test_explode() -> None:
    assert explode([[[[[4, 3], 4], 4], [7, [[8, 4], 9]]], [1, 1]]) == \
        ([[[[0, 7], 4], [7, [[8, 4], 9]]], [1, 1]], True)
    assert explode([[[[0, 7], 4], [7, [[8, 4], 9]]], [1, 1]]) == \
        ([[[[0, 7], 4], [15, [0, 13]]], [1, 1]], True)
    assert explode([[[[0, 7], 4], [[7, 8], [0, [6, 7]]]], [1, 1]]) == \
        ([[[[0, 7], 4], [[7, 8], [6, 0]]], [8, 1]], True)


def test_split() -> None:
    assert split([[[[0, 7], 4], [15, [0, 13]]], [1, 1]]) == \
        ([[[[0, 7], 4], [[7, 8], [0, 13]]], [1, 1]], True)
    assert split([[[[0, 7], 4], [[7, 8], [0, 13]]], [1, 1]]) == \
        ([[[[0, 7], 4], [[7, 8], [0, [6, 7]]]], [1, 1]], True)
    assert split([[[[0, 7], 4], [[7, 8], [0, [6, 7]]]], [1, 1]]) == \
        ([[[[0, 7], 4], [[7, 8], [0, [6, 7]]]], [1, 1]], False)


def test_add() -> None:
    assert add([[[[4, 3], 4], 4], [7, [[8, 4], 9]]], [1, 1]) == \
        [[[[0, 7], 4], [[7, 8], [6, 0]]], [8, 1]]


def test_magnitude() -> None:
    nums = parse_input(input2)
    assert magnitude(nums[0]) == 143
    assert magnitude(nums[1]) == 1384
    assert magnitude(nums[2]) == 445
    assert magnitude(nums[3]) == 791
    assert magnitude(nums[4]) == 1137
    assert magnitude(nums[5]) == 3488


def test_part_one() -> None:
    data = parse_input(input)
    assert part_one_calc(data) == 4140


def test_part_two() -> None:
    data = parse_input(input)
    assert part_two_calc(data) == 3993
