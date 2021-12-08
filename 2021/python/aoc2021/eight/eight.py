# AoC 2021 day eight

from dataclasses import dataclass

from ..lib.input import read_raw_input_data


@dataclass
class Display:
    examples: list[set[str]]
    segments: list[str]


def sort_str(s: str) -> str:
    # sort a string: "bcda" -> "abcd"
    return "".join(sorted(s))


def parse_input(input: str) -> list[Display]:
    displays = []
    for line in filter(None, input.splitlines()):
        parts = line.split(" | ")
        displays.append(
            Display(
                [set(p) for p in parts[0].split()],
                [p for p in parts[1].split()]
            ))
    return displays


def part_one_calc(data: list[Display]) -> int:
    def incl(n: str) -> int:
        return 1 if len(n) in (2, 3, 4, 7) else 0
    return sum([sum([incl(s) for s in d.segments]) for d in data])


digits = {
    "abcefg": "0",
    "cf": "1",
    "acdeg": "2",
    "acdfg": "3",
    "bcdf": "4",
    "abdfg": "5",
    "abdefg": "6",
    "acf": "7",
    "abcdefg": "8",
    "abcdfg": "9",
}


def segs_to_num(segs: list[str]) -> int:
    return int("".join([digits[sort_str(s)] for s in segs]))


def decode_wire_map(examples: list[set[str]]) -> dict[str, str]:
    # create the wire map based on a series of deductions
    d1 = [d for d in examples if len(d) == 2][0]
    d4 = [d for d in examples if len(d) == 4][0]
    d7 = [d for d in examples if len(d) == 3][0]
    d8 = [d for d in examples if len(d) == 7][0]
    # 7 - 1 tells me segment a
    seg_a = d7 - d1
    # 3 must have segments of 1, be 5 long
    d3 = [d for d in examples if len(d) == 5 and d.intersection(d1) == d1][0]
    # compare 3 and 4, remove a, c, f; common is d; b is in 4, and g is in 3  "abdg"
    seg_d = d4.intersection(d3) - d1
    seg_g = d3 - d4 - seg_a
    seg_b = d4 - d1 - seg_d
    # look at 8, I now know e -> "abdeg"
    seg_e = d8 - d3 - seg_b
    # look at nums with 6 -> one with abdeg is 6, remain is f. now I know "abcdefg"
    x = seg_a.union(seg_b, seg_d, seg_e, seg_g)
    seg_f = [(d - x) for d in examples if len(d) == 6 and len(d-x) == 1][0]
    seg_c = d1 - seg_f
    return {
        seg_a.pop(): "a",
        seg_b.pop(): "b",
        seg_c.pop(): "c",
        seg_d.pop(): "d",
        seg_e.pop(): "e",
        seg_f.pop(): "f",
        seg_g.pop(): "g",
    }


def part_two_calc(data: list[Display]) -> int:
    r = 0
    for d in data:
        wire_map = decode_wire_map(d.examples)
        tr = str.maketrans(wire_map)
        segs = [s.translate(tr) for s in d.segments]
        r += segs_to_num(segs)
    return r


def main() -> None:
    print("Day Eight")
    data = read_raw_input_data(__file__)
    displays = parse_input(data)
    print("Part One:", part_one_calc(displays))
    print("Part Two:", part_two_calc(displays))


if __name__ == "__main__":
    main()
