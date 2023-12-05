# Day Three Tests

from ..main import (
    calculate_gear_ratios,
    find_numbers_with_symbols,
    is_number_adjacent_to_symbol,
    is_symbol,
    parse_schematic,
)

example_schematic = """
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"""


def test_is_symbol():
    assert is_symbol("*")
    assert not is_symbol("4")
    assert not is_symbol(".")


def test_parse_schematic():
    grid = parse_schematic(example_schematic)
    assert len(grid) == 10
    assert len(grid[0]) == 10


def test_is_number_adjacent_to_symbol():
    grid = parse_schematic(example_schematic)
    assert is_number_adjacent_to_symbol(grid, 0, 0, "467")
    assert not is_number_adjacent_to_symbol(grid, 0, 5, "114")
    assert is_number_adjacent_to_symbol(grid, 2, 2, "35")


def test_find_numbers_with_symbols():
    grid = parse_schematic(example_schematic)
    numbers = find_numbers_with_symbols(grid)
    for n in numbers:
        print(n)
    assert (0, 0, "467", [(1, 3)]) in numbers
    assert (2, 2, "35", [(1, 3)]) in numbers
    assert (9, 1, "664", []) in numbers
    assert (9, 5, "598", [(8, 5)]) in numbers


def test_calculate_gear_ratios():
    grid = parse_schematic(example_schematic)
    numbers = find_numbers_with_symbols(grid)
    assert calculate_gear_ratios(numbers) == 16345 + 451490
