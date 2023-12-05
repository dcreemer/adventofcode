# Day Two Tests

from ..main import (
    calculate_possible_games_sum,
    calculate_power,
    calculate_total_power,
    min_cubes_for_game,
    parse_data,
)

SAMPLE_INPUT = """Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"""


def test_parse_data():
    games = parse_data(SAMPLE_INPUT)
    assert len(games) == 5
    assert games[0]["id"] == 1
    assert len(games[0]["subsets"]) == 3


def test_calculate_possible_games_sum():
    games = parse_data(SAMPLE_INPUT)
    result = calculate_possible_games_sum(games, 12, 13, 14)
    assert result == 8


# Sample Input as a Multiline String
SAMPLE_INPUT = """Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"""


def test_min_cubes_for_game():
    games = parse_data(SAMPLE_INPUT)
    min_red, min_green, min_blue = min_cubes_for_game(games[0])  # Game 1
    assert (min_red, min_green, min_blue) == (4, 2, 6)


def test_calculate_power():
    assert calculate_power(4, 2, 6) == 48


def test_calculate_total_power():
    games = parse_data(SAMPLE_INPUT)
    result = calculate_total_power(games)
    assert result == 2286
