# AoC 2021 day Sixteen

from dataclasses import dataclass, field
from functools import reduce
from operator import mul
from typing import Tuple

from ..lib.input import read_raw_input_data


@dataclass
class Packet:
    version: int
    tipe: int
    value: int = 0
    children: list["Packet"] = field(default_factory=list)


hmap = {
    "0": "0000",
    "1": "0001",
    "2": "0010",
    "3": "0011",
    "4": "0100",
    "5": "0101",
    "6": "0110",
    "7": "0111",
    "8": "1000",
    "9": "1001",
    "A": "1010",
    "B": "1011",
    "C": "1100",
    "D": "1101",
    "E": "1110",
    "F": "1111",
}


def hex_to_bin(hex_str: str) -> str:
    return "".join([hmap[c] for c in hex_str])


def consume(input: str, bits: int) -> Tuple[str, str]:
    return (input[0:bits], input[bits:])


def consume_int(data: str, bits: int) -> Tuple[int, str]:
    val, data = consume(data, bits)
    return (int(val, 2), data)


def consume_literal(data: str) -> Tuple[int, str]:
    val = []
    more = "1"
    while more == "1":
        more, data = consume(data, 1)
        b4, data = consume(data, 4)
        val.append(b4)
    _ = consume(data, 3)
    return (int("".join(val), 2), data)


def parse_packet(data: str) -> Tuple[Packet, str]:
    version, data = consume_int(data, 3)
    tipe, data = consume_int(data, 3)
    packet = Packet(version, tipe)
    if tipe == 4:
        # parse a literal packet:
        packet.value, data = consume_literal(data)
    else:
        # operator packet:
        lti, data = consume(data, 1)
        if lti == "0":
            subpbits, data = consume_int(data, 15)
            subpdata, data = consume(data, subpbits)
            while subpdata:
                subp, subpdata = parse_packet(subpdata)
                packet.children.append(subp)
        else:
            subpcnt, data = consume_int(data, 11)
            for i in range(subpcnt):
                subp, data = parse_packet(data)
                packet.children.append(subp)
    return packet, data


def sum_versions(ps: list[Packet]) -> int:
    return sum([sum_versions(p.children) + p.version for p in ps])


def part_one_calc(hex_data: str) -> int:
    data = hex_to_bin(hex_data)
    p, trailing = parse_packet(data)
    assert all(c == "0" for c in trailing)
    return sum_versions([p])


def evaluate(p: Packet) -> int:
    if p.tipe == 0:
        return sum([evaluate(c) for c in p.children])
    elif p.tipe == 1:
        return reduce(mul, [evaluate(c) for c in p.children], 1)
    elif p.tipe == 2:
        return min([evaluate(c) for c in p.children])
    elif p.tipe == 3:
        return max([evaluate(c) for c in p.children])
    elif p.tipe == 4:
        return p.value
    elif p.tipe == 5:
        return 1 if evaluate(p.children[0]) > evaluate(p.children[1]) else 0
    elif p.tipe == 6:
        return 1 if evaluate(p.children[0]) < evaluate(p.children[1]) else 0
    elif p.tipe == 7:
        return 1 if evaluate(p.children[0]) == evaluate(p.children[1]) else 0
    else:
        raise Exception("Unknown tipe: {}".format(p.tipe))


def part_two_calc(hex_data: str) -> int:
    data = hex_to_bin(hex_data)
    p, trailing = parse_packet(data)
    assert all(c == "0" for c in trailing)
    return evaluate(p)


def main() -> None:
    print("Day Sixteen")
    data = read_raw_input_data(__file__)
    print("Part One:", part_one_calc(data))
    print("Part Two:", part_two_calc(data))


if __name__ == "__main__":
    main()
