# AoC 2024 Day 04 Solution

from lib import files
from lib.parse import StringList

DAY = "04"
INPUT = files.read_to_lines(files.input_file(DAY))


Dirs: dict[str, list[tuple[int, int]]] = {
    "W": [(0, 0), (-1, 0), (-2, 0), (-3, 0)],
    "E": [(0, 0), (1, 0), (2, 0), (3, 0)],
    "N": [(0, 0), (0, -1), (0, -2), (0, -3)],
    "S": [(0, 0), (0, 1), (0, 2), (0, 3)],
    "NE": [(0, 0), (1, -1), (2, -2), (3, -3)],
    "NW": [(0, 0), (-1, -1), (-2, -2), (-3, -3)],
    "SE": [(0, 0), (1, 1), (2, 2), (3, 3)],
    "SW": [(0, 0), (-1, 1), (-2, 2), (-3, 3)],
}


def word_from_dir(matrix: StringList, ox: int, oy: int, steps: list[tuple[int, int]]) -> str | None:
    """
    given a matrix of characters with origin 0,0 at the top left, and a starting
    x, y position, construct a word from a direction. Return the word iff it
    is fully realizable withough overshooting the edge of the matrix
    """
    word: list[str] = []
    for dx, dy in steps:
        x, y = ox + dx, oy + dy
        if 0 <= x < len(matrix[0]) and 0 <= y < len(matrix):
            word.append(matrix[y][x])
    if len(word) == len(steps):
        return "".join(word)
    return None


def count_words_at(matrix: StringList, ox: int, oy: int, word: str) -> int:
    """
    given a matrix of characters with origin 0,0 at the top left, and a starting
    x, y position, count the number of words that can be constructed from all
    directions without overshooting the edge of the matrix. Return this count.
    """
    return sum(1 if word == word_from_dir(matrix, ox, oy, steps) else 0 for steps in Dirs.values())


def part_1(lines: StringList) -> int:
    return sum(
        count_words_at(lines, x, y, "XMAS") for x in range(len(lines[0])) for y in range(len(lines))
    )


def is_x_mas(matrix: StringList, ox: int, oy: int) -> bool:
    """
    return true if the given x, y coordinate is the center of an "X-MAS" --
    an "X" made from the work "MAS" crossing diagonally twice in any
    direction.
    """
    xmas = ("MAS", "SAM")
    return (
        word_from_dir(matrix, ox - 1, oy - 1, [(0, 0), (1, 1), (2, 2)]) in xmas
        and word_from_dir(matrix, ox + 1, oy - 1, [(0, 0), (-1, 1), (-2, 2)]) in xmas
    )


def part_2(lines: StringList) -> int:
    return sum(
        1 if is_x_mas(lines, x, y) else 0 for x in range(len(lines[0])) for y in range(len(lines))
    )


def main() -> None:
    print("Advent of Code Day", DAY)
    print("Part 1 Result: ", part_1(INPUT))
    print("Part 2 Result: ", part_2(INPUT))
