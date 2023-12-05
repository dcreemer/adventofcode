# AoC 2023 day three


def parse_schematic(schematic_str: str) -> list[list[str]]:
    """
    Parses the engine schematic from a string into a 2D grid.

    Args:
    schematic_str (str): The string representation of the engine schematic.

    Returns:
    List[List[str]]: A 2D grid representation of the engine schematic.
    """
    return [list(line) for line in schematic_str.strip().split("\n")]


def is_symbol(char: str) -> bool:
    """
    Checks if a character is a symbol.

    A symbol is defined as anything that isn't a digit or a period.

    Args:
    char (str): The character to check.

    Returns:
    bool: True if the character is a symbol, False otherwise.
    """
    return not char.isdigit() and char != "."


def find_numbers(grid: list[list[str]]) -> list[tuple[int, int, str]]:
    """
    Finds all numbers in the grid and their positions.

    Args:
    grid (List[List[str]]): The 2D grid representing the engine schematic.

    Returns:
    List[Tuple[int, int, str]]: A list of tuples containing row, column, and the number as a string.
    """
    numbers = []
    rows, cols = len(grid), len(grid[0])

    for row in range(rows):
        col = 0
        while col < cols:
            if grid[row][col].isdigit():
                number_start = col
                while col < cols and grid[row][col].isdigit():
                    col += 1
                numbers.append((row, number_start, "".join(grid[row][number_start:col])))
            else:
                col += 1
    return numbers


def find_numbers_with_symbols(
    grid: list[list[str]]
) -> list[tuple[int, int, str, list[tuple[int, int]]]]:
    """
    Finds all numbers in the grid, their positions, and positions of adjacent '*' symbols, including diagonally adjacent ones.

    Args:
    grid (List[List[str]]): The 2D grid representing the engine schematic.

    Returns:
    List[Tuple[int, int, str, List[Tuple[int, int]]]]: A list of tuples containing row, column,
                                                        the number as a string, and a list of positions of adjacent '*' symbols.
    """
    numbers = []
    rows, cols = len(grid), len(grid[0])

    for row in range(rows):
        col = 0
        while col < cols:
            if grid[row][col].isdigit():
                number_start = col
                adjacent_gears = set()
                while col < cols and grid[row][col].isdigit():
                    for dr in [-1, 0, 1]:
                        for dc in [-1, 0, 1]:
                            if dr == 0 and dc == 0:
                                continue  # Skip the number itself
                            adj_row, adj_col = row + dr, col + dc
                            if (
                                0 <= adj_row < rows
                                and 0 <= adj_col < cols
                                and grid[adj_row][adj_col] == "*"
                            ):
                                adjacent_gears.add((adj_row, adj_col))
                    col += 1
                numbers.append(
                    (row, number_start, "".join(grid[row][number_start:col]), list(adjacent_gears))
                )
            else:
                col += 1
    return numbers


def is_number_adjacent_to_symbol(grid: list[list[str]], row: int, start_col: int, number: str) -> bool:
    """
    Checks if a number in the grid is adjacent to a symbol.

    Args:
    grid (List[List[str]]): The 2D grid representing the engine schematic.
    row (int): The row index of the number.
    start_col (int): The starting column index of the number.
    number (str): The number as a string.

    Returns:
    bool: True if the number is adjacent to a symbol, False otherwise.
    """
    rows, cols = len(grid), len(grid[0])
    end_col = start_col + len(number)

    for r in range(row - 1, row + 2):
        for c in range(start_col - 1, end_col + 1):
            if r >= 0 and r < rows and c >= 0 and c < cols and is_symbol(grid[r][c]):
                return True
    return False


def calculate_gear_ratios(
    numbers_with_symbols: list[tuple[int, int, str, list[tuple[int, int]]]]
) -> int:
    """
    Calculates the sum of gear ratios for all identified gears.

    Args:
    numbers_with_symbols (List[Tuple[int, int, str, List[Tuple[int, int]]]]):
        The list of numbers and positions of their adjacent '*' symbols.

    Returns:
    int: The sum of all gear ratios.
    """
    gear_ratios = []
    gears_checked = set()

    # Mapping gears to the numbers they are adjacent to
    gear_to_numbers = {}
    for _, _, number, gears in numbers_with_symbols:
        for gear_pos in gears:
            if gear_pos not in gear_to_numbers:
                gear_to_numbers[gear_pos] = []
            gear_to_numbers[gear_pos].append(int(number))

    # Calculating gear ratios for gears adjacent to exactly two numbers
    for gear_pos, adjacent_numbers in gear_to_numbers.items():
        if len(adjacent_numbers) == 2 and gear_pos not in gears_checked:
            gears_checked.add(gear_pos)
            gear_ratios.append(adjacent_numbers[0] * adjacent_numbers[1])

    return sum(gear_ratios)


def sum_valid_numbers(grid: list[list[str]]) -> int:
    """
    Sums all valid numbers in the engine schematic.

    Args:
    grid (List[List[str]]): The 2D grid representing the engine schematic.

    Returns:
    int: The sum of all valid numbers.
    """
    total_sum = 0
    for row, col, number in find_numbers(grid):
        if is_number_adjacent_to_symbol(grid, row, col, number):
            total_sum += int(number)
    return total_sum


def main():
    with open("./aoc2023/three/input.txt") as file:
        schematic = file.read()
    grid = parse_schematic(schematic)
    result = sum_valid_numbers(grid)
    print(f"Sum of all part numbers: {result}")
    numbers = find_numbers_with_symbols(grid)
    gear_ratios = calculate_gear_ratios(numbers)
    print(f"Sum of gear ratios: {gear_ratios}")


if __name__ == "__main__":
    main()
