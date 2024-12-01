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
