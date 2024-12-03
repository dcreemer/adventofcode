# AoC 2024 Day 03 Solution

import re
from typing import override

from lib import files
from lib.parse import StringList

DAY = "03"
INPUT = files.read_to_lines(files.input_file(DAY))

InstructionsRE = re.compile(
    r"|".join(
        [
            r"(mul\((\d{1,3}),(\d{1,3})\))",
            r"(do\(\))",
            r"(don't\(\))",
        ]
    ),
    flags=re.MULTILINE,
)

# We'll use classes to represent each instruction


class Instruction:
    # for test assertions
    def __eq__(self, value: object) -> bool:
        return isinstance(value, self.__class__)


class Multiply(Instruction):
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y

    # for test assertions
    @override
    def __eq__(self, value: object) -> bool:
        if not isinstance(value, Multiply):
            return False
        return self.x == value.x and self.y == value.y


class Do(Instruction):
    pass


class Dont(Instruction):
    pass


def parse_input(data: str) -> list[Instruction]:
    """
    parse the input into a list of Instruction objects.
    """
    result: list[Instruction] = []
    for m in InstructionsRE.finditer(data):
        if m.group(0).startswith("don't()"):
            result.append(Dont())
        elif m.group(0).startswith("do"):
            result.append(Do())
        else:
            result.append(Multiply(int(m.group(2)), int(m.group(3))))
    return result


def part_1(lines: StringList) -> int:
    # sum all Multiply instructions
    return sum(m.x * m.y for m in parse_input("\n".join(lines)) if isinstance(m, Multiply))


def part_2(lines: StringList) -> int:
    # sum all Multiply instructions if `accum` is on.
    accum = True
    result = 0
    for i in parse_input("\n".join(lines)):
        if isinstance(i, Dont):
            accum = False
        elif isinstance(i, Do):
            accum = True
        elif accum and isinstance(i, Multiply):
            result += i.x * i.y
    return result


def main() -> None:
    print("Advent of Code Day", DAY)
    print("Part 1 Result: ", part_1(INPUT))
    print("Part 2 Result: ", part_2(INPUT))
