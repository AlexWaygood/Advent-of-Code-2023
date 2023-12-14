from dataclasses import dataclass
from typing import Self


@dataclass(slots=True, frozen=True, kw_only=True)
class Round:
    red: int = 0
    green: int = 0
    blue: int = 0


@dataclass(slots=True, frozen=True, kw_only=True)
class Game:
    game_id: int
    rounds: list[Round]


@dataclass(slots=True, frozen=True, kw_only=True)
class MinimumPossibleCubeSet:
    red: int
    green: int
    blue: int

    @classmethod
    def of_game(cls, game: Game) -> Self:
        rounds = iter(game.rounds)
        first_round = next(rounds)
        red, green, blue = first_round.red, first_round.green, first_round.blue
        for round in rounds:
            red = max(red, round.red)
            green = max(green, round.green)
            blue = max(blue, round.blue)
        return cls(red=red, green=green, blue=blue)

    def power(self) -> int:
        return self.red * self.green * self.blue


def parse_input_file(filename: str) -> list[Game]:
    games: list[Game] = []
    with open(filename) as f:
        for game_id, game_description in enumerate(f, start=1):
            if not game_description:
                continue
            round_descriptions = game_description.partition(": ")[-1]
            rounds: list[Round] = []
            for round_description in round_descriptions.split("; "):
                round_data: dict[str, int] = {}
                for colour_description in round_description.split(", "):
                    number_description, _, colour = colour_description.partition(" ")
                    round_data[colour.strip()] = int(number_description)
                rounds.append(Round(**round_data))
            games.append(Game(game_id=game_id, rounds=rounds))
    return games


def calculate(input_filename: str) -> int:
    games = parse_input_file(input_filename)
    return sum(MinimumPossibleCubeSet.of_game(game).power() for game in games)


def main(input_filename: str) -> None:
    print(calculate(input_filename))


if __name__ == "__main__":
    main("input.txt")
