# AoC 2021 day six


from ..lib.input import read_raw_input_data


def parse_input(input: str) -> list[int]:
    counts = [0] * 9
    for n in [int(n.strip()) for n in input.split(",")]:
        counts[n] += 1
    return counts


def age_one(fish_counts: list[int]) -> None:
    # fish_count is a count of how many fish are that many days from spanwing
    # It is an array that is always 9 elements long (the max days from spawning),
    # and is modified in place to age one day.
    new_fish = fish_counts[0]
    for i in range(8):
        fish_counts[i] = fish_counts[i+1]
    fish_counts[8] = new_fish
    fish_counts[6] += new_fish


def age(fish: list[int], generations: int) -> int:
    newfish = fish.copy()
    for _ in range(generations):
        age_one(newfish)
    return sum(newfish)


def part_one_calc(fish: list[int]) -> int:
    return age(fish, 80)


def part_two_calc(fish: list[int]) -> int:
    return age(fish, 256)


def main() -> None:
    print("Day Six")
    data = read_raw_input_data(__file__)
    fish = parse_input(data)
    print("Part One:", part_one_calc(fish))
    print("Part Two:", part_two_calc(fish))


if __name__ == "__main__":
    main()
