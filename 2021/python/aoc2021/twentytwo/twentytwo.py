# AoC 2021 day Twenty-Two

import re
from dataclasses import dataclass
from enum import Enum
from typing import Tuple

from ..lib.input import read_raw_input_data


@dataclass
class Rect:
    x_min: int
    x_max: int
    y_min: int
    y_max: int

    def intersects(self, other: "Rect") -> bool:
        return not (self.x_max < other.x_min or self.x_min > other.x_max or
                    self.y_max < other.y_min or self.y_min > other.y_max)

    def area(self) -> int:
        return (self.x_max - self.x_min + 1) * (self.y_max - self.y_min + 1)

    def __hash__(self) -> int:
        return hash((self.x_min, self.x_max, self.y_min, self.y_max))

    def __eq__(self, other: "Rect") -> bool:
        return self.x_min == other.x_min and self.x_max == other.x_max and \
                self.y_min == other.y_min and self.y_max == other.y_max

@dataclass
class Volume:
    x_min: int
    x_max: int
    y_min: int
    y_max: int
    z_min: int
    z_max: int

    def intersects(self, other: "Volume") -> bool:
        return not (self.x_max < other.x_min or self.x_min > other.x_max or
                    self.y_max < other.y_min or self.y_min > other.y_max or
                    self.z_max < other.z_min or self.z_min > other.z_max)

    def volume(self) -> int:
        return (self.x_max - self.x_min + 1) \
            * (self.y_max - self.y_min + 1)  \
            * (self.z_max - self.z_min + 1)

    def __hash__(self) -> int:
        return hash((self.x_min, self.x_max, self.y_min, self.y_max, self.z_min, self.z_max))

    def __eq__(self, other: "Volume") -> bool:
        return self.x_min == other.x_min and self.x_max == other.x_max and \
                self.y_min == other.y_min and self.y_max == other.y_max and \
                self.z_min == other.z_min and self.z_max == other.z_max


Step = Tuple[str, Volume]


def parse_input(input: str) -> list[Step]:
    vals = []
    for line in [line for line in input.splitlines() if line]:
        # on x=10..12,y=10..12,z=10..12
        m = re.match(r"(\w+) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)", line)
        if m is not None:
            vals.append((
                m.group(1),
                Volume(
                    int(m.group(2)),
                    int(m.group(3)),
                    int(m.group(4)),
                    int(m.group(5)),
                    int(m.group(6)),
                    int(m.group(7))
                )))
    return vals


Coords = set[Tuple[int, int, int]]


def process_step(cube: Coords, step: Step) -> None:
    on = step[0] == "on"
    vol = step[1]
    for x in range(vol.x_min, vol.x_max + 1):
        for y in range(vol.y_min, vol.y_max + 1):
            for z in range(vol.z_min, vol.z_max + 1):
                if on:
                    cube.add((x, y, z))
                else:
                    cube.discard((x, y, z))


class Overlap(Enum):
    NONE = 0
    A_RIGHT_B = 2
    A_LEFT_B = 3
    A_IN_B = 4
    B_IN_A = 5


def choose_split(a_min: int, a_max: int, b_min: int, b_max: int) -> Tuple[Overlap, int]:
    # return a comparision of the two line segments:
    # 0 -> no overlap; don't split
    # 1 -> split giving split to A
    # 2 -> split giving split to B
    # 3 -> A completely inside B; don't split
    if a_max < b_min or b_max < a_min:
        return (Overlap.NONE, 0)
    if b_min <= a_min <= a_max <= b_max:
        return (Overlap.A_IN_B, 0)
    if a_min < b_min <= b_max <= a_max:
        return (Overlap.B_IN_A, b_min - 1)
    if b_min <= a_min <= b_max < a_max:
        return (Overlap.A_RIGHT_B, b_max)
    if a_min < b_min <= a_max:
        return (Overlap.A_LEFT_B, b_min - 1)
    raise ValueError(f"unexpected case: {a_min} {a_max} {b_min} {b_max}")


def subtract_rect(a: Rect, b: Rect) -> list[Rect]:
    print("sub", a, "-", b)
    vals = []
    ovl, x_split = choose_split(a.x_min, a.x_max, b.x_min, b.x_max)
    print("X SPLIT", ovl, x_split)
    if ovl in (Overlap.A_LEFT_B, Overlap.A_RIGHT_B, Overlap.B_IN_A):
        a1 = Rect(a.x_min, x_split, a.y_min, a.y_max)
        a2 = Rect(x_split + 1, a.x_max, a.y_min, a.y_max)
        print("xa1", a1)
        print("xa2", a2)
        if b.intersects(a1):
            print("xa1 needs more")
            vals.extend(subtract_rect(a1, b))
        else:
            print("xa1 is done")
            vals.append(a1)
        if b.intersects(a2):
            print("xa2 needs more")
            vals.extend(subtract_rect(a2, b))
        else:
            print("xa2 is done")
            vals.append(a2)
    elif ovl == Overlap.NONE:
        vals.append(a)
    elif ovl == Overlap.A_IN_B:
        pass
    else:
        raise ValueError(f"unexpected case: {ovl}")
    ovl, y_split = choose_split(a.y_min, a.y_max, b.y_min, b.y_max)
    print("Y SPLIT", ovl, x_split)
    if ovl in (Overlap.A_LEFT_B, Overlap.A_RIGHT_B, Overlap.B_IN_A):
        a1 = Rect(a.x_min, a.x_max, a.y_min, y_split)
        a2 = Rect(a.x_min, a.x_max, y_split + 1, a.y_max)
        print("ya1", a1)
        print("ya2", a2)
        if b.intersects(a1):
            print("ya1 needs more")
            vals.extend(subtract_rect(a1, b))
        else:
            print("ya1 is done")
            vals.append(a1)
        if b.intersects(a2):
            print("ya2 needs more")
            vals.extend(subtract_rect(a2, b))
        else:
            print("ya2 is done")
            vals.append(a2)
    elif ovl == Overlap.NONE:
        vals.append(a)
    elif ovl == Overlap.A_IN_B:
        pass
    print("RET:", vals)
    return vals


def subtract_volume(a: Volume, b: Volume) -> list[Volume]:
    """
    Subtract b from a. This may "chop" A into two parts: the non-overlapping
    part and the overlapping part. We add the overlapping part to the result,
    and then subtract b from the overlapping part.
    """
    print("sub", a, "-", b)
    vals = []
    ovl, x_split = choose_split(a.x_min, a.x_max, b.x_min, b.x_max)
    if ovl in (Overlap.A_LEFT_B, Overlap.A_RIGHT_B, Overlap.B_IN_A):
        a1 = Volume(a.x_min, x_split, a.y_min, a.y_max, a.z_min, a.z_max)
        a2 = Volume(x_split + 1, a.x_max, a.y_min, a.y_max, a.z_min, a.z_max)
        print("xa1", a1)
        print("xa2", a2)
        if b.intersects(a1):
            print("xa1 needs more")
            vals.extend(subtract_volume(a1, b))
        else:
            print("xa1 is done")
            vals.append(a1)
        if b.intersects(a2):
            print("xa2 needs more")
            vals.extend(subtract_volume(a2, b))
        else:
            print("xa2 is done")
            vals.append(a2)
    elif ovl == Overlap.NONE:
        vals.append(a)
    elif ovl == Overlap.A_IN_B:
        pass
    else:
        raise ValueError(f"unexpected case: {ovl}")
    ovl, y_split = choose_split(a.y_min, a.y_max, b.y_min, b.y_max)
    if ovl in (Overlap.A_LEFT_B, Overlap.A_RIGHT_B, Overlap.B_IN_A):
        a1 = Volume(a.x_min, a.x_max, a.y_min, y_split, a.z_min, a.z_max)
        a2 = Volume(a.x_min, a.x_max, y_split + 1, a.y_max, a.z_min, a.z_max)
        print("ya1", a1)
        print("ya2", a2)
        if b.intersects(a1):
            print("ya1 needs more")
            vals.extend(subtract_volume(a1, b))
        else:
            print("ya1 is done")
            vals.append(a1)
        if b.intersects(a2):
            print("ya2 needs more")
            vals.extend(subtract_volume(a2, b))
        else:
            print("ya2 is done")
            vals.append(a2)
    elif ovl == Overlap.NONE:
        vals.append(a)
    elif ovl == Overlap.A_IN_B:
        pass
    ovl, z_split = choose_split(a.z_min, a.z_max, b.z_min, b.z_max)
    if ovl in (Overlap.A_LEFT_B, Overlap.A_RIGHT_B, Overlap.B_IN_A):
        a1 = Volume(a.x_min, a.x_max, a.y_min, a.y_max, a.z_min, z_split)
        a2 = Volume(a.x_min, a.x_max, a.y_max, a.y_max, z_split + 1, a.z_max)
        print("za1", a1)
        print("za2", a2)
        if b.intersects(a1):
            print("za1 needs more")
            vals.extend(subtract_volume(a1, b))
        else:
            print("za1 is done")
            vals.append(a1)
        if b.intersects(a2):
            print("za2 needs more")
            vals.extend(subtract_volume(a2, b))
        else:
            print("za2 is done")
            vals.append(a2)
    elif ovl == Overlap.NONE:
        vals.append(a)
    elif ovl == Overlap.A_IN_B:
        pass
    return vals


def part_one_calc(data: list[Step]) -> int:
    cube: Coords = set()
    for step in data:
        v = step[1]
        if any(c < -50 for c in (v.x_min, v.y_min, v.z_min)) or \
                any(c > 50 for c in (v.x_max, v.y_max, v.z_max)):
            continue
        process_step(cube, step)
    return len(cube)


def part_one_calc2(data: list[Step]) -> int:
    # must always start with an "on" volume
    assert data[0][0] == "on"
    vols: list[Volume] = [data[0][1]]
    for step in data[1:]:
        v = step[1]
        if any(c < -50 for c in (v.x_min, v.y_min, v.z_min)) or \
                any(c > 50 for c in (v.x_max, v.y_max, v.z_max)):
            continue
        if step[0] == "on":
            # to add V into the list, if intersects with any A for A in the existing, remove that A and
            # replace it with A-V parts. Then finally add V
            # then add those to the list.
            i = 0
            while i < len(vols):
                a = vols[i]
                if a.intersects(v):
                    a_parts = subtract_volume(a, v)
                    vols[i:i + 1] = a_parts
                else:
                    vols.append(v)
                    i += 1
        if step[0] == "off":
            # to subtract V from the list, if intersects with any A for A in the existing,
            # remove that A and replace it with A-V parts.
            i = 0
            while i < len(vols):
                a = vols[i]
                if a.intersects(v):
                    a_parts = subtract_volume(a, v)
                    vols[i:i + 1] = a_parts
                else:
                    i += 1
    return sum(v.volume() for v in vols)


def part_two_calc(data: list[Step]) -> int:
    return 2


def main() -> None:
    print("Day Twenty-Two")
    data = read_raw_input_data(__file__)
    stuff = parse_input(data)
    print("Part One:", part_one_calc(stuff))
    # print("Part Two:", part_two_calc(stuff))


if __name__ == "__main__":
    main()
