# utilities to read files
import pathlib

from .parse import StringList, strip_lines


def input_file(day: str) -> pathlib.Path:
    """
    get the input file name for a day
    """
    return pathlib.Path(f"data/day{day}_input.txt").resolve()


def read_to_lines(src: pathlib.Path) -> StringList:
    """
    read a data file into lines, stripping whitespace and empty lines
    """
    data = src.read_text()
    return strip_lines(data)


def read_to_two_lines(src: pathlib.Path) -> tuple[StringList, StringList]:
    """
    read a data file into two sets of lines, stripping whitespace and empty lines
    the two sets are split when double newlines are found.
    """
    data = src.read_text().split("\n\n")
    return strip_lines(data[0]), strip_lines(data[1])
