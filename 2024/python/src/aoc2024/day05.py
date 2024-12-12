# AoC 2024 Day 05 Solution
import itertools

from lib import files
from lib.parse import StringList, parse_list_to_int_lists

DAY = "05"
INPUT = files.read_to_two_lines(files.input_file(DAY))

type RulesSet = set[tuple[int, int]]
type PagesList = list[list[int]]


def build_rules_set(lines: StringList) -> RulesSet:
    result = set()
    for line in lines:
        (a, b) = map(int, line.split("|"))
        result.add((a, b))
    return result


def build_pages_lists(lines: StringList) -> PagesList:
    return parse_list_to_int_lists(lines, ",")


def part_1(rules: RulesSet, pages_list: PagesList) -> int:
    result = 0
    for pages in pages_list:
        # for every combination a, b in the list, included the result
        # if and only if B|A is not in the rules set.
        if all((b, a) not in rules for a, b in itertools.pairwise(pages)):
            result += pages[len(pages) // 2]
    return result


def part_2(rules: RulesSet, pages_list: PagesList) -> int:
    result = 0
    for pages in pages_list:
        if all((b, a) not in rules for a, b in itertools.pairwise(pages)):
            # skip properly ordered pages
            continue
        # put the pages list in the correct order using a bubble insertion:
        again = True
        while again:
            again = False
            for a, b in itertools.pairwise(pages):
                if (b, a) in rules:
                    ai = pages.index(a)
                    bi = pages.index(b)
                    pages[ai], pages[bi] = pages[bi], pages[ai]
                    again = True
        result += pages[len(pages) // 2]
    return result


def main() -> None:
    print("Advent of Code Day", DAY)
    rules = build_rules_set(INPUT[0])
    pages = build_pages_lists(INPUT[1])
    print("Part 1 Result: ", part_1(rules, pages))
    print("Part 2 Result: ", part_2(rules, pages))
