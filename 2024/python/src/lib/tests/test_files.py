# test files functions
from ..files import input_file


def test_input_file() -> None:
    assert input_file("01").resolve().as_posix().endswith("data/day01_input.txt")
