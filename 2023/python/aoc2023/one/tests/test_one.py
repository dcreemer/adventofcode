# tests for day one

from ..main import parse_input

input = """199
200
208
210
200
207
240
260
269
263"""


def test_parse_input() -> None:
    x = parse_input(input)
    assert len(x) == 10
