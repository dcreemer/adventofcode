# AoC 2021 day Twenty

from typing import Tuple

from ..lib.input import read_raw_input_data


class Image:
    def __init__(self, data: list[str], background: str) -> None:
        # we make each image have a 1-pixel extra border
        buffer_w = len(data[0]) + 2
        self.data = [background * buffer_w]
        for row in data:
            self.data.append(background + row + background)
        self.data.append(background * buffer_w)
        self.width = len(self.data[0])
        self.height = len(self.data)
        self.background = background

    def get_pixel(self, x: int, y: int) -> str:
        if y < 0 or y >= self.height or x < 0 or x >= self.width:
            return self.background
        return self.data[y][x]

    def set_pixel(self, x: int, y: int, pixel: str) -> None:
        self.data[y] = self.data[y][:x] + pixel + self.data[y][x+1:]

    def get_code(self, x: int, y: int) -> int:
        bits = []
        for dx, dy in [(dx, dy) for dy in range(-1, 2) for dx in range(-1, 2)]:
            bits.append("0" if self.get_pixel(x+dx, y+dy) == "." else "1")
        return int("".join(bits), 2)

    def get_pixel_count(self, pixel: str) -> int:
        return sum(row.count(pixel) for row in self.data)

    def enhance(self, bits: str) -> "Image":
        # create a new image with +1 dimensions in each direction
        # and set the image shifted down & to the right by 1
        # if the "background" pixel at bits[0] is "#", then out background
        # will toggle on and off for each enhance cycle
        if self.background == "." and bits[0] == "#":
            background = "#"
        else:
            background = "."
        new_img = Image(self.data, background)
        for y in range(self.height):
            for x in range(self.width):
                c = self.get_code(x, y)
                new_img.set_pixel(x+1, y+1, bits[c])
        return new_img

    def __str__(self) -> str:
        return "\n".join(self.data)


def parse_input(input: str) -> Tuple[str, Image]:
    a, b = input.split("\n\n")
    bits = a.replace("\n", "").strip()
    image = Image([line for line in b.split("\n") if line], ".")
    return bits, image


def part_one_calc(bits: str, img: Image) -> int:
    for _ in range(2):
        img = img.enhance(bits)
    return img.get_pixel_count("#")


def part_two_calc(bits: str, img: Image) -> int:
    for _ in range(50):
        img = img.enhance(bits)
    return img.get_pixel_count("#")


def main() -> None:
    print("Day Twenty")
    data = read_raw_input_data(__file__)
    bits, img = parse_input(data)
    print("Part One:", part_one_calc(bits, img))
    print("Part Two:", part_two_calc(bits, img))


if __name__ == "__main__":
    main()
