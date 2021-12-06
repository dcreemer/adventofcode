# AoC 2021 day fives

import re
from dataclasses import dataclass

from ..lib.input import read_raw_input_data


@dataclass
class Line:
    x1: int
    y1: int
    x2: int
    y2: int


def parse_input(input: str) -> list[Line]:
    lines = []
    for line in [line for line in input.splitlines() if line]:
        m = re.match(r"(\d+),(\d+) -> (\d+),(\d+)", line)
        if m:
            lines.append(Line(int(m.group(1)), int(m.group(2)), int(m.group(3)), int(m.group(4))))
        else:
            raise ValueError("Unable to parse line: {line}".format(line=line))
    return lines


class Grid:
    def __init__(self, lines: list[Line]):
        max_x = max([max(line.x1 for line in lines), max(line.x2 for line in lines)]) + 1
        max_y = max([max(line.y1 for line in lines), max(line.y2 for line in lines)]) + 1
        self.grid: list[list[int]] = [[0 for _ in range(max_x)] for _ in range(max_y)]

    def apply_line(self, line: Line) -> None:
        # "Draw" a line segment on the grid. Coords are inclusive.
        start_x = line.x1
        end_x = line.x2
        start_y = line.y1
        end_y = line.y2
        if line.x1 < line.x2:
            end_x = line.x2 + 1
            x_inc = 1
        elif line.x1 > line.x2:
            end_x = line.x2 - 1
            x_inc = -1
        else:
            x_inc = 0
        if line.y1 < line.y2:
            end_y = line.y2 + 1
            y_inc = 1
        elif line.y1 > line.y2:
            end_y = line.y2 - 1
            y_inc = -1
        else:
            y_inc = 0
        x = start_x
        y = start_y
        while (x_inc != 0 and x != end_x) or (y_inc != 0 and y != end_y):
            self.grid[y][x] += 1
            x += x_inc
            y += y_inc

    def count_overlaps(self) -> int:
        return sum(sum(map(lambda c: 1 if c > 1 else 0, row)) for row in self.grid)

    def __str__(self) -> str:
        return "\n".join(["".join(map(lambda c: "%d" % c if c > 0 else ".", row)) for row in self.grid])


def part_one_calc(lines: list[Line]) -> int:
    g = Grid(lines)
    for line in lines:
        # only apply horizontal lines or vertical lines
        if line.x1 == line.x2 or line.y1 == line.y2:
            g.apply_line(line)
    return g.count_overlaps()


def part_two_calc(lines: list[Line]) -> int:
    g = Grid(lines)
    for line in lines:
        g.apply_line(line)
    return g.count_overlaps()


def main() -> None:
    print("Day Five")
    data = read_raw_input_data(__file__)
    lines = parse_input(data)
    print("Part One:", part_one_calc(lines))
    print("Part Two:", part_two_calc(lines))


if __name__ == "__main__":
    main()
