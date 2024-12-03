# AoC 2024 Day 02 Solution

from lib import files, parse
from lib.parse import StringList

DAY = "02"
INPUT = files.read_to_lines(files.input_file(DAY))


def is_valid_report(report: list[int]) -> bool:
    """
    a valid report is always strictly ascending or descending,
    with a delta between numbers of 1 <= delta <= 4
    """
    deltas = [report[i] - report[i - 1] for i in range(1, len(report))]
    return all(delta > 0 and delta < 4 for delta in deltas) or all(
        delta > -4 and delta < 0 for delta in deltas
    )


def part_1(lines: StringList) -> int:
    reports = parse.parse_list_to_int_matrix(lines)
    return sum([1 if is_valid_report(r) else 0 for r in reports])


def is_valid_damped_report(report: list[int]) -> bool:
    """
    a valid /damped/ report is always strictly ascending or descending,
    with a delta between numbers of 1 <= delta <= 4, or *could* be if one
    single report number were removed
    """
    if is_valid_report(report):
        return True
    for i in range(len(report)):
        damped = report[:]
        del damped[i]
        if is_valid_report(damped):
            return True
    return False


def part_2(lines: StringList) -> int:
    reports = parse.parse_list_to_int_matrix(lines)
    return sum([1 if is_valid_damped_report(r) else 0 for r in reports])


def main() -> None:
    print("Advent of Code Day", DAY)
    print("Part 1 Result: ", part_1(INPUT))
    print("Part 2 Result: ", part_2(INPUT))
