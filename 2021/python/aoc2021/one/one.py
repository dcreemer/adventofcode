# AoC 2021 day one

import collections
from itertools import islice
from typing import Any, Generator, Iterable, Tuple


def parse_input(input: str) -> list[int]:
    return [int(n) for n in input.splitlines() if n]


def sliding_window(iterable: Iterable[Any], n: int) -> Generator[Tuple[Any, ...], None, None]:
    # from https://docs.python.org/3/library/itertools.html
    # sliding_window('ABCDEFG', 4) -> ABCD BCDE CDEF DEFG
    it = iter(iterable)
    window = collections.deque(islice(it, n), maxlen=n)
    if len(window) == n:
        yield tuple(window)
    for x in it:
        window.append(x)
        yield tuple(window)


def count_increases(depths: list[int]) -> int:
    return sum([1 if b > a else 0 for a, b in zip(depths, depths[1:])])


def part_one_calc(depths: list[int]) -> int:
    return count_increases(depths)


def part_two_calc(depths: list[int]) -> int:
    depths = [sum(d) for d in sliding_window(depths, 3)]
    return count_increases(depths)


def main() -> None:
    print("Day One")
    with open("./aoc2021/one/input.txt", "r") as f:
        data = f.read()
    depths = parse_input(data)
    print("Part One:", part_one_calc(depths))
    print("Part Two:", part_two_calc(depths))


if __name__ == "__main__":
    main()
