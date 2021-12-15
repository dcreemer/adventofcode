# AoC 2021 day Fourteen

from collections import Counter, defaultdict
# itertools pairwise is in Python 3.10+, but mypy doesn't know about it yet
from itertools import pairwise  # type: ignore
from typing import DefaultDict, Tuple

from ..lib.input import read_raw_input_data

Pair = Tuple[str, str]
Rules = dict[Pair, str]


def parse_input(input: str) -> Tuple[str, Rules]:
    poly, x = input.split("\n\n")
    print(x)
    rules = {(x[0], x[1]): x[-1] for x in x.split("\n") if x}
    return poly, rules


def solve(poly_in: str, rules: Rules, rounds: int) -> int:
    # arrrg. I spent 20 minutes recognizing that this line:

    # `poly = {(a, b): 1 for a, b in pairwise(poly_in)}`

    # is not the same as these three, as the first one assumes
    # that the input string has only distinct pairs. Of course,
    # the test cases do not have that issue...
    poly: DefaultDict[Pair, int] = defaultdict(int)
    for p in pairwise(poly_in):
        poly[p] += 1

    for i in range(rounds):
        npoly: DefaultDict[Pair, int] = defaultdict(int)
        for pair in poly:
            npoly[(pair[0], rules[pair])] += poly[pair]
            npoly[(rules[pair], pair[1])] += poly[pair]
        poly = npoly.copy()
    results: Counter[str] = Counter()
    for (a, b), v in poly.items():
        results[a] += v
        results[b] += v
    # the very first and last letters only appear once
    results[poly_in[0]] += 1
    results[poly_in[-1]] += 1
    # sort from most to least common:
    si = results.most_common()
    return (si[0][1] - si[-1][1]) // 2


def part_one_calc(poly_in: str, rules: Rules) -> int:
    return solve(poly_in, rules, 10)


def part_two_calc(poly_in: str, rules: Rules) -> int:
    return solve(poly_in, rules, 40)


def main() -> None:
    print("Day Fourteen")
    data = read_raw_input_data(__file__)
    poly, rules = parse_input(data)
    print("Part One:", poly, part_one_calc(poly, rules))
    # print("Part Two:", poly, part_two_calc(poly, rules))


if __name__ == "__main__":
    main()
