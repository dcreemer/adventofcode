# AoC 2021 day eleven

from typing import Tuple

from ..lib.input import read_raw_input_data


def parse_input(input: str) -> list[list[int]]:
    return [[int(x) for x in list(line)] for line in input.splitlines() if line]


def neighbors(x: int, y: int, data: list[list[int]]) -> list[Tuple[int, int]]:
    result = []
    for dx in range(-1, 2):
        for dy in range(-1, 2):
            if dx == 0 and dy == 0:
                continue
            if x + dx >= 0 and x + dx < len(data[0]) and y + dy >= 0 and y + dy < len(data):
                result.append((x + dx, y + dy))
    return result


def flash(data: list[list[int]]) -> int:
    # flash elements that are at 10, and update neighbors
    # return number of flashes
    flashes = 0
    for y in range(len(data)):
        for x in range(len(data[0])):
            if data[y][x] >= 10:
                data[y][x] = 0
                flashes += 1
                for nx, ny in neighbors(x, y, data):
                    if data[ny][nx] > 0:
                        data[ny][nx] += 1
    return flashes


def step(data: list[list[int]]) -> int:
    # increase every element by 1
    for y in range(len(data)):
        for x in range(len(data[0])):
            data[y][x] += 1
    # check for flashes. keep going until no more flashes
    flashes = 0
    new_flashes = 1
    while new_flashes > 0:
        new_flashes = flash(data)
        flashes += new_flashes
    return flashes


def part_one_calc(data: list[list[int]]) -> int:
    return sum(step(data) for n in range(100))


def all_zero(data: list[list[int]]) -> int:
    return sum(sum(line) for line in data) == 0


def part_two_calc(data: list[list[int]]) -> int:
    n = 0
    while not all_zero(data):
        n += 1
        step(data)
    return n


def main() -> None:
    print("Day Eleven")
    data = read_raw_input_data(__file__)
    print("Part One:", part_one_calc(parse_input(data)))
    print("Part Two:", part_two_calc(parse_input(data)))


if __name__ == "__main__":
    main()
