# AoC 2021 day four

import re
from dataclasses import dataclass
from typing import Tuple

from ..lib.input import read_raw_input_data

# dict for number -> row, col
# 2d matrix for row, col -> bool (true if called)
# for each num, check if in dict. if so, mark matrix as called
# to check for win:
#   any(all(row) for row in matrix) or any(all(col) for col in matrix)


@dataclass
class Coord:
    row: int
    col: int


class Card:
    def __init__(self, data: str):
        self.nums: dict[int, Coord] = {}
        self.matrix: list[list[bool]] = [[False for _ in range(5)] for _ in range(5)]
        self.parse(data)

    def parse(self, data: str) -> None:
        for row, line in enumerate(data.splitlines()):
            for col, num in enumerate(re.split(r'\s+', line.strip())):
                n = int(num)
                self.nums[n] = Coord(row, col)

    def mark_called(self, num: int) -> None:
        if num in self.nums:
            coord = self.nums[num]
            self.matrix[coord.row][coord.col] = True

    def check_win(self) -> bool:
        # zip(*matrix) is the transpose of the matrix
        return any(all(row) for row in self.matrix) or any(all(col) for col in zip(*self.matrix))

    def sum_uncalled(self) -> int:
        return sum(n for n, coord in self.nums.items() if not self.matrix[coord.row][coord.col])


def parse_input(input: str) -> Tuple[list[int], list[Card]]:
    lines = input.split("\n\n")
    return ([int(i) for i in lines[0].split(",")], [Card(line) for line in lines[1:]])


def part_one_calc(nums: list[int], boards: list[Card]) -> int:
    for num in nums:
        for board in boards:
            board.mark_called(num)
            if board.check_win():
                return num * board.sum_uncalled()
    raise ValueError("No solution found")


def part_two_calc(nums: list[int], boards: list[Card]) -> int:
    for num in nums:
        for board in boards:
            board.mark_called(num)
            if all(b.check_win() for b in boards):
                return num * board.sum_uncalled()
    raise ValueError("No solution found")


def main() -> None:
    print("Day Four")
    data = read_raw_input_data(__file__)
    nums, cards = parse_input(data)
    print("Part One:", part_one_calc(nums, cards))
    print("Part Two:", part_two_calc(nums, cards))


if __name__ == "__main__":
    main()
