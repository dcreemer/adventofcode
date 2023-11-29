# AoC 2023 day one


def parse_input(input: str) -> list[int]:
    return [int(n) for n in input.splitlines() if n]


def main() -> None:
    print("Day One")
    with open("./aoc2023/one/input.txt") as f:
        data = f.read()
    parsed = parse_input(data)
    print("Part One:", len(parsed))
    print("Part Two:", len(parsed))


if __name__ == "__main__":
    main()
