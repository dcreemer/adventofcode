# tests for day Twenty

from ..twenty import parse_input, part_one_calc, part_two_calc

input = """
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
"""


def test_parse_input() -> None:
    bits, img = parse_input(input)
    assert len(bits) == 512
    assert img.get_pixel_count("#") == 10
    assert img.get_code(2+1, 2+1) == 34
    img.set_pixel(2+1, 3+1, ".")
    assert img.data[3+1] == "......."


def test_part_one() -> None:
    bits, img = parse_input(input)
    assert part_one_calc(bits, img) == 35


def test_part_two() -> None:
    bits, img = parse_input(input)
    assert part_two_calc(bits, img) == 3351
