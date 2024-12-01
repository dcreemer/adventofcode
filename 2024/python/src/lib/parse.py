# AoC library functions

from collections.abc import Sequence


def strip_lines(src: str) -> list[str]:
    """
    read a data file into lines, stripping whitespace and empty lines
    """
    return [line for line in [y.strip() for y in src.split("\n")] if line]


def parse_list_to_ints(src: Sequence[str]) -> Sequence[int]:
    """
    Parse a list of strings into a list of integers.
    Ignore blank linkes and all trailing or leading whitespace.
    """
    return [int(x) for x in [y.strip() for y in src] if x]


def parse_list_to_int_matrix(src: Sequence[str]) -> list[list[int]]:
    """
    parse a list of strings into a matrix (2D array) of integers.
    """
    return [[int(x) for x in line.split(" ")] for line in src]
