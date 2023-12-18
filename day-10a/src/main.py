from collections.abc import Mapping
from dataclasses import dataclass
from enum import Enum, auto
from typing import NamedTuple


class Direction(Enum):
    NORTH = auto()
    SOUTH = auto()
    EAST = auto()
    WEST = auto()


NORTH = Direction.NORTH
SOUTH = Direction.SOUTH
EAST = Direction.EAST
WEST = Direction.WEST


class Pipe(Enum):
    NORTH_SOUTH = NORTH, SOUTH
    SOUTH_EAST = SOUTH, EAST
    EAST_WEST = EAST, WEST
    NORTH_WEST = NORTH, WEST
    SOUTH_WEST = SOUTH, WEST
    NORTH_EAST = NORTH, EAST


INPUT_TO_PIPE = {
    "|": Pipe.NORTH_SOUTH,
    "-": Pipe.EAST_WEST,
    "L": Pipe.NORTH_EAST,
    "J": Pipe.NORTH_WEST,
    "7": Pipe.SOUTH_WEST,
    "F": Pipe.SOUTH_EAST,
    # Aka Ground: no pipe on this tile
    ".": None,
}


class Coordinates(NamedTuple):
    x: int
    y: int


@dataclass(frozen=True, kw_only=True, slots=True)
class PuzzleInput:
    pipe_map: Mapping[Coordinates, Pipe | None]
    start_coordinates: Coordinates


def solve(puzzle_input: PuzzleInput) -> int:
    steps = 1
    start_coords = puzzle_input.start_coordinates
    coords = start_coords._replace(y=start_coords.y-1)
    previous_movement = NORTH
    seen_coordinates = {coords}

    while coords != start_coords:
        steps += 1
        node = puzzle_input.pipe_map[coords]
        match (node, previous_movement):
            case [Pipe.NORTH_SOUTH, Direction.NORTH]:
                coords = coords._replace(y=coords.y-1)
                previous_movement = NORTH
            case [Pipe.NORTH_SOUTH, Direction.SOUTH]:
                coords = coords._replace(y=coords.y+1)
                previous_movement = SOUTH
            case [Pipe.EAST_WEST, Direction.EAST]:
                coords = coords._replace(x=coords.x+1)
                previous_movement = EAST
            case [Pipe.EAST_WEST, Direction.WEST]:
                coords = coords._replace(x=coords.x-1)
                previous_movement = WEST
            case [Pipe.SOUTH_EAST, Direction.NORTH]:
                coords = coords._replace(x=coords.x+1)
                previous_movement = EAST
            case [Pipe.SOUTH_EAST, Direction.WEST]:
                coords = coords._replace(y=coords.y+1)
                previous_movement = SOUTH
            case [Pipe.NORTH_WEST, Direction.SOUTH]:
                coords = coords._replace(x=coords.x-1)
                previous_movement = WEST
            case [Pipe.NORTH_WEST, Direction.EAST]:
                coords = coords._replace(y=coords.y-1)
                previous_movement = NORTH
            case [Pipe.SOUTH_WEST, Direction.NORTH]:
                coords = coords._replace(x=coords.x-1)
                previous_movement = WEST
            case [Pipe.SOUTH_WEST, Direction.EAST]:
                coords = coords._replace(y=coords.y+1)
                previous_movement = SOUTH
            case [Pipe.NORTH_EAST, Direction.WEST]:
                coords = coords._replace(y=coords.y-1)
                previous_movement = NORTH
            case [Pipe.NORTH_EAST, Direction.SOUTH]:
                coords = coords._replace(x=coords.x+1)
                previous_movement = EAST
            case _:
                assert False, (node, previous_movement)
        assert coords not in seen_coordinates, node
        seen_coordinates.add(coords)
        node = puzzle_input.pipe_map[coords]

    return steps // 2


def parse_input(filename: str) -> PuzzleInput:
    pipe_map: dict[Coordinates, Pipe | None] = {}
    start_coordinates: Coordinates | None = None
    with open(filename) as f:
        for y, line in enumerate(f):
            for x, char in enumerate(line.strip()):
                if char == "S":
                    pipe = Pipe.NORTH_SOUTH
                    coordinates = Coordinates(x, y)
                    start_coordinates = coordinates
                else:
                    pipe = INPUT_TO_PIPE[char]
                    coordinates = Coordinates(x, y)
                pipe_map[coordinates] = pipe

    assert start_coordinates is not None
    return PuzzleInput(
        pipe_map=pipe_map, start_coordinates=start_coordinates
    )


def main() -> None:
    puzzle_input = parse_input("input.txt")
    print(solve(puzzle_input))


if __name__ == "__main__":
    main()