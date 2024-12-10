# unittest for the calculate module

from typing import cast

from ..parse import StringList, parse_list_to_int_matrix, parse_list_to_ints, strip_lines


def test_strip_lines() -> None:
    src = """
    1
    2
    3   
    4   
    """  # noqa
    assert strip_lines(src) == cast(StringList, ["1", "2", "3", "4"])


def test_int_lists() -> None:
    src = """
    1
    2
    3
    4
    """
    assert parse_list_to_ints(strip_lines(src)) == [1, 2, 3, 4]


def test_int_matrix() -> None:
    src = """
    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9
    """
    assert parse_list_to_int_matrix(strip_lines(src)) == [
        [7, 6, 4, 2, 1],
        [1, 2, 7, 8, 9],
        [9, 7, 6, 2, 1],
        [1, 3, 2, 4, 5],
        [8, 6, 4, 4, 1],
        [1, 3, 6, 7, 9],
    ]


def test_char_matrix() -> None:
    src = """
    MMMSXXMASM
    MSAMXMSMSA
    AMXSXMAAMM
    MSAMASMSMX
    XMASAMXAMM
    XXAMMXXAMA
    SMSMSASXSS
    SAXAMASAAA
    MAMMMXMMMM
    MXMXAXMASX
    """
    matrix = strip_lines(src)
    assert len(matrix) == 10  # rows
    assert len(matrix[0]) == 10  # columns
    #             Y  X
    assert matrix[0][0] == "M"
    assert matrix[0][1] == "M"
    assert matrix[0][2] == "M"
    assert matrix[1][1] == "S"
    assert matrix[2][9] == "M"
