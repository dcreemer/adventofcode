# tests for day Fourteen

from ..fourteen import parse_input, part_one_calc, part_two_calc

input = """NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"""


def test_parse_input() -> None:
    poly, rules = parse_input(input)
    assert len(rules) == 16


def test_part_one() -> None:
    poly, rules = parse_input(input)
    assert part_one_calc(poly, rules) == 1588


def test_part_two() -> None:
    poly, rules = parse_input(input)
    assert part_two_calc(poly, rules) == 2188189693529
