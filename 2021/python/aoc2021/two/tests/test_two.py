# tests for day two

from ..two import navigate_one, navigate_two, parse_input, part_one_calc, part_two_calc

input = """forward 5
down 5
forward 8
up 3
down 8
forward 2"""


def test_parse_input() -> None:
    x = parse_input(input)
    assert len(x) == 6


def test_navigate_one() -> None:
    commands = parse_input(input)
    assert navigate_one(commands) == (15, 10)


def test_part_one() -> None:
    commands = parse_input(input)
    assert part_one_calc(commands) == 150


def test_navigate_two() -> None:
    commands = parse_input(input)
    assert navigate_two(commands) == (15, 60)


def test_part_two() -> None:
    commands = parse_input(input)
    assert part_two_calc(commands) == 900
