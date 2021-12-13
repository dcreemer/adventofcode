# AoC 2021 day Thirteen

from typing import Tuple

from pydantic import BaseModel

from ..lib.input import read_raw_input_data


class Coord(BaseModel):
    x: int
    y: int


class Fold(BaseModel):
    axis: str
    val: int


def parse_input(input: str) -> Tuple[list[Coord], list[Fold]]:
    a, b = input.split("\n\n")
    dots = [Coord(x=x, y=y) for x, y in [line.split(",") for line in a.split("\n") if line]]
    folds = [Fold(axis=d, val=n) for d, n in [f[11:].split("=") for f in b.split("\n") if f]]
    return dots, folds


# Perhaps could be a class, with xy get/set fns, and __init__ and __str__
# Alternatively use Numpy and rotate matrix to do column-fold
Matrix = list[list[str]]


def create_matrix(dots: list[Coord]) -> Matrix:
    dim_x = max(d.x for d in dots) + 1
    dim_y = max(d.y for d in dots) + 1
    matrix = [["." for _ in range(dim_x)] for _ in range(dim_y)]
    for dot in dots:
        matrix[dot.y][dot.x] = "#"
    return matrix


def matrix_to_str(matrix: Matrix) -> str:
    return "\n".join("".join(row) for row in matrix)


def fold_x(matrix: Matrix, n: int) -> Matrix:
    result = [[matrix[r][c] for c in range(n)] for r in range(len(matrix))]
    for src_c in range(n+1, len(matrix[0])):
        target_c = n - (src_c - n)
        for i in range(len(matrix)):
            dot = matrix[i][src_c]
            if dot == "#":
                result[i][target_c] = "#"
    return result


def fold_y(matrix: Matrix, n: int) -> Matrix:
    result = matrix.copy()[:n]
    for src_r in range(n+1, len(matrix)):
        target_r = n - (src_r - n)
        for j in range(len(matrix[0])):
            dot = matrix[src_r][j]
            if dot == "#":
                result[target_r][j] = "#"
    return result


def part_one_calc(dots: list[Coord], fold: Fold) -> int:
    matrix = create_matrix(dots)
    folded = fold_y(matrix, fold.val) if fold.axis == "y" else fold_x(matrix, fold.val)
    return sum(1 if folded[y][x] == "#" else 0 for x in range(len(folded[0])) for y in range(len(folded)))


def part_two_calc(dots: list[Coord], folds: list[Fold]) -> str:
    matrix = create_matrix(dots)
    for fold in folds:
        matrix = fold_y(matrix, fold.val) if fold.axis == "y" else fold_x(matrix, fold.val)
    return matrix_to_str(matrix)


def main() -> None:
    print("Day Thirteen")
    data = read_raw_input_data(__file__)
    dots, folds = parse_input(data)
    print("Part One:", part_one_calc(dots, folds[0]))
    print("Part Two:")
    print(part_two_calc(dots, folds))


if __name__ == "__main__":
    main()
