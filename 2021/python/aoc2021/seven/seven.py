# AoC 2021 day seven

import statistics

from ..lib.input import read_raw_input_data


def parse_input(input: str) -> list[int]:
    return [int(n.strip()) for n in input.split(",") if n]


def part_one_calc(data: list[int]) -> int:
    med = int(statistics.median(data))

    def cost(n: int) -> int:
        return abs(n-med)

    return sum([cost(n) for n in data])


def part_two_calc(data: list[int]) -> int:

    def cost(v: int, n: int) -> int:
        d = abs(n-v)
        return d * (d+1) // 2

    return min([sum([cost(i, n) for n in data]) for i in range(max(data))])


def main() -> None:
    print("Day Seven")
    data = read_raw_input_data(__file__)
    positions = parse_input(data)
    print("Part One:", part_one_calc(positions))
    print("Part Two:", part_two_calc(positions))


if __name__ == "__main__":
    main()
