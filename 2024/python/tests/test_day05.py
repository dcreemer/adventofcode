# tests for day01 functions

from aoc2024 import day05, parse

sample_input = """
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"""


def test_rules_set() -> None:
    rules_lines = parse.strip_two_lines(sample_input)[0]
    rules = day05.build_rules_set(rules_lines)
    assert len(rules) == 21
    assert (47, 53) in rules
    assert (97, 61) in rules
    assert (29, 13) in rules
    assert (47, 75) not in rules


def test_pages_list() -> None:
    pages_lines = parse.strip_two_lines(sample_input)[1]
    pages = day05.build_pages_lists(pages_lines)
    assert len(pages) == 6
    assert pages[0] == [75, 47, 61, 53, 29]
    assert pages[1] == [97, 61, 53, 29, 13]
    assert pages[2] == [75, 29, 13]
    assert pages[3] == [75, 97, 47, 61, 53]
    assert pages[4] == [61, 13, 29]
    assert pages[5] == [97, 13, 75, 29, 47]


def test_part_1() -> None:
    lines = parse.strip_two_lines(sample_input)
    rules = day05.build_rules_set(lines[0])
    pages = day05.build_pages_lists(lines[1])
    result = day05.part_1(rules, pages)
    assert result == 143


def test_part_2() -> None:
    lines = parse.strip_two_lines(sample_input)
    rules = day05.build_rules_set(lines[0])
    pages = day05.build_pages_lists(lines[1])
    result = day05.part_2(rules, pages)
    assert result == 123


def test_version() -> None:
    assert day05.DAY == "05"
