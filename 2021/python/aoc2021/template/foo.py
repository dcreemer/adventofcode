# AoC 2021 day XYZ

from ..lib.input import read_raw_input_data


def parse_input(input: str) -> list[str]:
    return list()


def part_one_calc(data: list[str]) -> int:
    return 1


def part_two_calc(data: list[str]) -> int:
    return 2


def main() -> None:
    print("Day XYZ")
    data = read_raw_input_data(__file__)
    stuff = parse_input(data)
    print("Part One:", part_one_calc(stuff))
    print("Part Two:", part_two_calc(stuff))


if __name__ == "__main__":
    main()
