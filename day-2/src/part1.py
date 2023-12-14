from dataclasses import dataclass
from typing import Final


@dataclass(slots=True, frozen=True, kw_only=True)
class Round:
    red: int = 0
    green: int = 0
    blue: int = 0

    def total_cubes(self) -> int:
        return self.red + self.green + self.blue


@dataclass(slots=True, frozen=True, kw_only=True)
class Game:
    game_id: int
    rounds: list[Round]


CONSTRAINTS: Final = Round(red=12, green=13, blue=14)


def game_was_possible(game: Game) -> bool:
    return all(
        (
            round.red <= CONSTRAINTS.red
            and round.green <= CONSTRAINTS.green
            and round.blue <= CONSTRAINTS.blue
            and round.total_cubes() <= CONSTRAINTS.total_cubes()
        )
        for round in game.rounds
    )


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
    return sum(game.game_id for game in games if game_was_possible(game))


def main(input_filename: str) -> None:
    print(calculate(input_filename))


if __name__ == "__main__":
    main("input.txt")
