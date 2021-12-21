# AoC 2021 day Twenty-one
# this solution also owes the insight of swapping params on part 2 to
# the reddit AoC forum. Prior to that I was doing much more verbose
# passing of params and comparing results for player 1 and player 2 separately.
# using the param swap allows for a nicer solution since the same code is used
# andthe same results are memoized

import functools
from typing import Tuple

from ..lib.input import read_raw_input_data


def parse_input(input: str) -> Tuple[int, int]:
    a, b = input.strip().split("\n")
    return int(a[-1]), int(b[-1])


BoardLen = 10
det_die = 1
det_die_rolls = 0


def det_roll() -> int:
    global det_die, det_die_rolls
    result = det_die
    det_die += 1
    if det_die > 100:
        det_die = 1
    det_die_rolls += 1
    return result


def part_one_calc(pos_1: int, pos_2: int) -> int:
    score_1 = 0
    score_2 = 0
    while True:
        rolls = det_roll() + det_roll() + det_roll()
        pos_1 = (pos_1 + rolls - 1) % BoardLen + 1
        score_1 += pos_1
        if score_1 >= 1000:
            return score_2 * det_die_rolls
        rolls = det_roll() + det_roll() + det_roll()
        pos_2 = (pos_2 + rolls - 1) % BoardLen + 1
        score_2 += pos_2
        if score_2 >= 1000:
            return score_1 * det_die_rolls


# memoize the results of calling this function. Lots of universe states
# will result in the same win counts.
@functools.cache
def count_wins(pos_1: int, score_1: int, pos_2: int, score_2: int) -> Tuple[int, int]:
    if score_1 >= 21:
        return 1, 0
    if score_2 >= 21:
        return 0, 1
    wins_1 = 0
    wins_2 = 0
    # generate all dice roll possibilities and add up the wins in the sub-universes
    for (d, e, f) in [(d, e, f) for d in range(1, 3+1) for e in range(1, 3+1) for f in range(1, 3+1)]:
        # calc pos & score for player 1
        p = (pos_1 + d + e + f - 1) % BoardLen + 1
        s = score_1 + p
        # recur to do the same for other player (hence swap of pos_1 and pos_2)
        # this caches the result of each call. Whatever the position of the players and
        # the scores of players, we will always get the same result. So cache it and
        # cutoff the brute force search
        sub_win_2, sub_win_1 = count_wins(pos_2, score_2, p, s)
        # accumulate the wins from the sub-universies
        wins_1 += sub_win_1
        wins_2 += sub_win_2
    return wins_1, wins_2


def part_two_calc(pos_a: int, pos_b: int) -> int:
    return max(count_wins(pos_a, 0, pos_b, 0))


def main() -> None:
    print("Day Twenty-one")
    data = read_raw_input_data(__file__)
    a, b = parse_input(data)
    print("Part One:", part_one_calc(a, b))
    print("Part Two:", part_two_calc(a, b))


if __name__ == "__main__":
    main()
