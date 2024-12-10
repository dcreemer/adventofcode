# tests for day04 functions

from aoc2024 import day04, parse

sample_input = """
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


def test_word_from_dir() -> None:
    data = parse.strip_lines(sample_input)
    assert day04.word_from_dir(data, 6, 4, day04.Dirs["W"]) == "XMAS"
    assert day04.word_from_dir(data, 0, 4, day04.Dirs["E"]) == "XMAS"
    assert day04.word_from_dir(data, 9, 3, day04.Dirs["S"]) == "XMAS"
    assert day04.word_from_dir(data, 6, 5, day04.Dirs["NW"]) == "XMAS"
    assert day04.word_from_dir(data, 9, 3, day04.Dirs["SW"]) == "XMAS"


def test_count_words_at() -> None:
    data = parse.strip_lines(sample_input)
    assert day04.count_words_at(data, 6, 4, "XMAS") == 2


def test_part_1() -> None:
    result = day04.part_1(parse.strip_lines(sample_input))
    assert result == 18


def test_is_x_mas() -> None:
    data = parse.strip_lines(sample_input)
    assert day04.is_x_mas(data, 2, 1)


def test_part_2() -> None:
    result = day04.part_2(parse.strip_lines(sample_input))
    assert result == 9


def test_version() -> None:
    assert day04.DAY == "04"
