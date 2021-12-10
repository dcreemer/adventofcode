# AoC 2021 day ten

from typing import Union

from ..lib.input import read_raw_input_data

CValues = {
    ')': 3,
    ']': 57,
    '}': 1197,
    '>': 25137,
}

IValues = {
    '(': 1,
    '[': 2,
    '{': 3,
    '<': 4,
}


def check_syntax(data: str) -> Union[str, list[str]]:
    stack = []
    for char in data:
        # print(char, stack)
        if char in '({[<':
            stack.append(char)
        elif char == ')' and stack[-1] != '(':
            return ')'
        elif char == ']' and stack[-1] != '[':
            return ']'
        elif char == '}' and stack[-1] != '{':
            return '}'
        elif char == '>' and stack[-1] != '<':
            return '>'
        else:
            stack.pop()
    return stack


def parse_input(input: str) -> list[str]:
    return [line for line in input.splitlines() if line]


def part_one_calc(data: list[str]) -> int:
    result = 0
    for line in data:
        score = check_syntax(line)
        if isinstance(score, str):
            result += CValues[score]
    return result


def part_two_calc(data: list[str]) -> int:
    results: list[int] = []
    for line in data:
        rest = check_syntax(line)
        if isinstance(rest, list):
            result = 0
            for c in reversed(rest):
                result *= 5
                result += IValues[c]
            results.append(result)
    return sorted(results)[len(results) // 2]


def main() -> None:
    print("Day Ten")
    data = read_raw_input_data(__file__)
    stuff = parse_input(data)
    print("Part One:", part_one_calc(stuff))
    print("Part Two:", part_two_calc(stuff))


if __name__ == "__main__":
    main()
