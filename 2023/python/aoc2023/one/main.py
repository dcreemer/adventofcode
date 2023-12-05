# AoC 2023 day one

import re


def read_file_content(file_path: str) -> str:
    """
    Reads the entire content of a file into a string.

    :param file_path: Path to the file.
    :return: String containing the entire content of the file.
    """
    with open(file_path) as file:
        return file.read()


def parse_calibration_data(data: str) -> list[str]:
    """
    Parses the string containing calibration data into a list of lines.

    :param data: String containing the calibration data.
    :return: List of lines in the calibration data.
    """
    return data.splitlines()


digit_mapping = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}


def parse_line_to_digits(line: str) -> list[int]:
    """
    Parses a line and returns a list of digits (both numerical and spelled-out ones) allowing for overlapping words.

    :param line: A string representing a line from the calibration data.
    :return: List of integers representing the digits found in the line.
    """
    regex_pattern = r"(?=(one|two|three|four|five|six|seven|eight|nine|\d))"
    matches = re.finditer(regex_pattern, line)

    digits = []
    for match in matches:
        digit = match.group(1)
        if digit.isdigit():
            digits.append(int(digit))
        else:
            digits.append(digit_mapping[digit])

    return digits


def calculate_calibration_sum_one(lines: list[str]) -> int:
    """
    Calculates the sum of calibration values from the given lines.

    :param lines: List of strings representing the calibration data.
    :return: Sum of all calibration values.
    """
    total_sum = 0
    for line in lines:
        digits = [char for char in line if char.isdigit()]
        if digits:
            calibration_value = int(digits[0] + digits[-1])
            total_sum += calibration_value
    return total_sum


def calculate_calibration_sum_two(lines: list[str]) -> int:
    """
    Calculates the sum of calibration values from the given lines, accounting for spelled-out digits.

    :param lines: List of strings representing the calibration data.
    :return: Sum of all calibration values.
    """
    total_sum = 0
    for line in lines:
        digits = parse_line_to_digits(line)
        if digits:
            calibration_value = int(str(digits[0]) + str(digits[-1]))
            # print(f"YES: {line}, {digits}, {calibration_value}")
            total_sum += calibration_value
        else:
            print(f"NO: {line}")
    return total_sum


def main():
    """
    Main program to solve the puzzle.
    """
    file_content = read_file_content("./aoc2023/one/input.txt")
    lines = parse_calibration_data(file_content)
    print(f"Part 1: {calculate_calibration_sum_one(lines)}")
    print(f"Part 2: {calculate_calibration_sum_two(lines)}")


if __name__ == "__main__":
    main()


# def parse_input(input: str) -> list[int]:
#     return [int(n) for n in input.splitlines() if n]


# def main() -> None:
#     print("Day One")
#     with open("./aoc2023/one/input.txt") as f:
#         data = f.read()
#     parsed = parse_input(data)
#     print("Part One:", len(parsed))
#     print("Part Two:", len(parsed))


# if __name__ == "__main__":
#     main()
