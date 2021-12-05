# AoC 2021 day two

from dataclasses import dataclass
from typing import Tuple


@dataclass
class Command:
    direction: str
    amount: int


def parse_input(input: str) -> list[Command]:
    return [Command(d, int(a)) for d, a in [line.split(" ") for line in input.splitlines()]]


def navigate_one(commands: list[Command]) -> Tuple[int, int]:
    # return position, depth
    hpos, depth = 0, 0
    for command in commands:
        if command.direction == "forward":
            hpos += command.amount
        elif command.direction == "down":
            depth += command.amount
        elif command.direction == "up":
            depth -= command.amount
        else:
            raise ValueError(f"Unknown direction {command.direction}")
    return hpos, depth


def part_one_calc(depths: list[Command]) -> int:
    hpos, depth = navigate_one(depths)
    return hpos * depth


def navigate_two(commands: list[Command]) -> Tuple[int, int]:
    # return position, depth
    hpos, depth, aim = 0, 0, 0
    for command in commands:
        if command.direction == "forward":
            hpos += command.amount
            depth += (aim * command.amount)
        elif command.direction == "down":
            aim += command.amount
        elif command.direction == "up":
            aim -= command.amount
        else:
            raise ValueError(f"Unknown direction {command.direction}")
    return hpos, depth


def part_two_calc(depths: list[Command]) -> int:
    hpos, depth = navigate_two(depths)
    return hpos * depth


def main() -> None:
    print("Day Two")
    with open("./aoc2021/two/input.txt", "r") as f:
        data = f.read()
    depths = parse_input(data)
    print("Part One:", part_one_calc(depths))
    print("Part Two:", part_two_calc(depths))


if __name__ == "__main__":
    main()
