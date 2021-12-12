# AoC 2021 day Twelve

from collections import Counter
from typing import Callable

from ..lib.input import read_raw_input_data

Path = list[str]
Graph = dict[str, list[str]]


def parse_input(input: str) -> Graph:
    result: dict[str, list[str]] = {}
    for line in input.splitlines():
        if line:
            k, v = line.split("-")
            result.setdefault(k, []).append(v)
            result.setdefault(v, []).append(k)
    return result


def should_visit_one(c: str, path: Path) -> bool:
    return c.isupper() or c not in path


def should_visit_two(c: str, path: Path) -> bool:
    if c == "start":
        return False
    if c.isupper():
        return True
    if c not in path:
        return True
    cnt = Counter([p for p in path if p.islower()])
    return all(v == 1 for v in cnt.values())


def traverse(
    graph: Graph,
    cur: Path,
    results: list[Path],
    vfn: Callable[[str, Path], bool]
) -> None:
    n = cur[-1]
    if n == "end":
        results.append(cur)
        return
    for child in [c for c in graph.get(n, []) if vfn(c, cur)]:
        new_path = cur.copy()
        new_path.append(child)
        traverse(graph, new_path, results, vfn)


def part_one_calc(data: Graph) -> int:
    paths: list[Path] = []
    traverse(data, ["start"], paths, should_visit_one)
    return len(paths)


def part_two_calc(data: Graph) -> int:
    paths: list[Path] = []
    traverse(data, ["start"], paths, should_visit_two)
    return len(paths)


def main() -> None:
    print("Day Twelve")
    data = read_raw_input_data(__file__)
    stuff = parse_input(data)
    print("Part One:", part_one_calc(stuff))
    print("Part Two:", part_two_calc(stuff))


if __name__ == "__main__":
    main()
