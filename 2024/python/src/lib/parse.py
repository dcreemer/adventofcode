# AoC library functions

from collections.abc import Sequence
from typing import cast


class StringList(Sequence[str]):
    """
    A StringList is a list of strings, guaranteed to have no blank lines, and
    no leading or trailing blanks per line.
    """


def strip_lines(src: str) -> StringList:
    """
    read a data file into lines, stripping whitespace and empty lines
    """
    return cast(StringList, [line for line in [y.strip() for y in src.split("\n")] if line])


def parse_list_to_ints(src: StringList) -> Sequence[int]:
    """
    Parse a list of strings into a list of integers.
    Ignore blank linkes and all trailing or leading whitespace.
    """
    return [int(x) for x in src]


def parse_list_to_int_matrix(src: StringList) -> list[list[int]]:
    """
    parse a list of strings into a matrix (2D array) of integers.
    """
    return [[int(x) for x in line.split(" ")] for line in src]
