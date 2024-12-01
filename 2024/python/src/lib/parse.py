# AoC library functions

import pathlib
from collections.abc import Sequence


def read_file_to_lines(src: pathlib.Path) -> list[str]:
    with src.open() as f:
        return f.readlines()


def parse_list_to_ints(src: Sequence[str]) -> Sequence[int]:
    """
    parse a list of strings into a list of integers
    """
    return [int(x.strip()) for x in src if x.strip()]
