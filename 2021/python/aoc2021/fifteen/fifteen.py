# AoC 2021 day Fifteen

import heapq
import sys
from collections import defaultdict, namedtuple
from typing import Tuple, cast

from ..lib.input import read_raw_input_data

Matrix = list[list[int]]
Coord = namedtuple("Coord", ["x", "y"])


def parse_input(input: str) -> Matrix:
    return [[int(n) for n in line] for line in input.split("\n") if line]


# simple wrapper for the input matrix to understand the concept
# of edge costs and node neighbors
class Graph:
    def __init__(self, matrix: Matrix) -> None:
        self.matrix = matrix
        self.dim_y = len(matrix)
        self.dim_x = len(matrix[0])

    def edge_cost(self, _from_node: Coord, to_node: Coord) -> int:
        # the "cost" of an edge is the distance between the nodes, which in our
        # case is just the value at the to_node
        return cast(int, self.matrix[to_node.y][to_node.x])

    def neighbors(self, node: Coord) -> list[Coord]:
        value = []
        if node.x > 0:
            value.append(Coord(node.x - 1, node.y))
        if node.x < self.dim_x - 1:
            value.append(Coord(node.x + 1, node.y))
        if node.y > 0:
            value.append(Coord(node.x, node.y - 1))
        if node.y < self.dim_y - 1:
            value.append(Coord(node.x, node.y + 1))
        return value


def shortest_path(graph: Graph, start_node: Coord) -> dict[Coord, int]:
    """
    calculates the cost of shortest paths from start_node to all other nodes
    in a graph using Dijkstra's algorithm
    """
    # unvisited is a priority queue of (cost, node) tuples
    unvisited: list[Tuple[int, Coord]] = [(0, start_node)]
    heapq.heapify(unvisited)
    # costs maps nodes to the cost of the shortest path to that node
    # by default, the cost is "infinite", except for the start node
    costs: dict[Coord, int] = defaultdict(lambda: sys.maxsize)
    costs[start_node] = 0
    # do the BFS
    while unvisited:
        # get the current lowest cost node:
        cur_weight, cur_node = heapq.heappop(unvisited)
        # and look at its neighbors:
        for n in graph.neighbors(cur_node):
            weight = cur_weight + graph.edge_cost(cur_node, n)
            # if the weight to this neighbor is lower than the current weight,
            # update the cost and path
            if weight < costs[n]:
                heapq.heappush(unvisited, (weight, n))
                costs[n] = weight
    return costs


def part_one_calc(matrix: Matrix) -> int:
    g = Graph(matrix)
    start = Coord(0, 0)
    end = Coord(g.dim_x - 1, g.dim_y - 1)
    costs = shortest_path(g, start)
    return costs[end]


def expand(matrix: Matrix, factor: int) -> Matrix:
    def bump(v: int, inc: int) -> int:
        v = v + inc
        return v - 9 if v > 9 else v
    dim_x = len(matrix[0])
    dim_y = len(matrix)
    new_matrix = [[0 for _ in range(dim_x * factor)] for _ in range(dim_y * factor)]
    # expand to the right:
    for i in range(factor):
        for y in range(dim_y):
            for x in range(dim_x):
                new_matrix[y][i*dim_x + x] = bump(matrix[y][x], i)
    # expand down:
    for i in range(1, factor):
        for y in range(dim_y):
            for x in range(len(new_matrix[0])):
                new_matrix[i*dim_y + y][x] = bump(new_matrix[y][x], i)
    return new_matrix


def part_two_calc(matrix: Matrix) -> int:
    g = Graph(expand(matrix, 5))
    start = Coord(0, 0)
    end = Coord(g.dim_x - 1, g.dim_y - 1)
    costs = shortest_path(g, start)
    return costs[end]


def main() -> None:
    print("Day Fifteen")
    data = read_raw_input_data(__file__)
    stuff = parse_input(data)
    print("Part One:", part_one_calc(stuff))
    print("Part Two:", part_two_calc(stuff))


if __name__ == "__main__":
    main()
