# AoC 2021 day three

from typing import Callable

from ..lib.input import read_raw_input_data


def parse_input(input: str) -> list[str]:
    return [line for line in input.splitlines() if line]


def mcb(bits: list[int]) -> int:
    mid = len(bits) / 2
    return 1 if sum(bits) >= mid else 0


def lcb(bits: list[int]) -> int:
    mid = len(bits) / 2
    return 1 if sum(bits) < mid else 0


def part_one_calc(bits: list[str]) -> int:
    width = len(bits[0])
    epsilon, gamma = "", ""
    for c in range(width):
        col = [int(b[c]) for b in bits]
        epsilon += str(mcb(col))
        gamma += str(lcb(col))
    return int(epsilon, 2) * int(gamma, 2)


def bit_filter(fn: Callable[[list[int]], int], bits: list[str]) -> int:
    width = len(bits[0])
    for c in range(width):
        col = [int(b[c]) for b in bits]
        mc = str(fn(col))
        bits = [b for b in bits if b[c] == mc]
        if len(bits) == 1:
            return int(bits[0], 2)
    raise ValueError("No bits found")


def part_two_calc(bits: list[str]) -> int:
    oxy = bit_filter(mcb, bits)
    co2 = bit_filter(lcb, bits)
    return oxy * co2


def main() -> None:
    print("Day Three")
    data = read_raw_input_data(__file__)
    bits = parse_input(data)
    print("Part One:", part_one_calc(bits))
    print("Part Two:", part_two_calc(bits))


if __name__ == "__main__":
    main()
