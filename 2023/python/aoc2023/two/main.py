# AoC 2023 day two


def min_cubes_for_game(game: dict[str, list[str]]) -> tuple[int, int, int]:
    min_red, min_green, min_blue = 0, 0, 0
    for subset in game["subsets"]:
        red, green, blue = 0, 0, 0
        for color_info in subset.split(", "):
            number, color = color_info.split(" ")
            if color == "red":
                red = int(number)
            elif color == "green":
                green = int(number)
            elif color == "blue":
                blue = int(number)
        min_red = max(min_red, red)
        min_green = max(min_green, green)
        min_blue = max(min_blue, blue)
    return min_red, min_green, min_blue


def calculate_power(red: int, green: int, blue: int) -> int:
    return red * green * blue


def calculate_total_power(games: list[dict[str, list[str]]]) -> int:
    total_power = 0
    for game in games:
        min_red, min_green, min_blue = min_cubes_for_game(game)
        total_power += calculate_power(min_red, min_green, min_blue)
    return total_power


def read_data(file_path):
    with open(file_path) as file:
        return file.read()


def parse_data(data):
    games = []
    for line in data.split("\n"):
        if line.startswith("Game"):
            game_id, details = line.split(": ")
            subsets = details.split("; ")
            games.append({"id": int(game_id.split(" ")[1]), "subsets": subsets})
    return games


def is_game_possible(game, red, green, blue):
    for subset in game["subsets"]:
        counts = {"red": 0, "green": 0, "blue": 0}
        for color_info in subset.split(", "):
            number, color = color_info.split(" ")
            counts[color] += int(number)
        if counts["red"] > red or counts["green"] > green or counts["blue"] > blue:
            return False
    return True


def calculate_possible_games_sum(games, max_red, max_green, max_blue):
    sum_ids = 0
    for game in games:
        if is_game_possible(game, max_red, max_green, max_blue):
            sum_ids += game["id"]
    return sum_ids


def main():
    print("Day Two")
    data = read_data("./aoc2023/two/input.txt")
    games = parse_data(data)
    max_red, max_green, max_blue = 12, 13, 14
    result = calculate_possible_games_sum(games, max_red, max_green, max_blue)
    print("Part 1:", result)
    total_power = calculate_total_power(games)
    print("Part 2:", total_power)


if __name__ == "__main__":
    main()
