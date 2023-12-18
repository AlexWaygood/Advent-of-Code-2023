from __future__ import annotations

from collections.abc import Mapping
from dataclasses import dataclass
from enum import Enum, auto
from typing import Final


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


INPUT_TO_PIPE: Final[Mapping[str, Pipe]] = {
    "|": Pipe.NORTH_SOUTH,
    "-": Pipe.EAST_WEST,
    "L": Pipe.NORTH_EAST,
    "J": Pipe.NORTH_WEST,
    "7": Pipe.SOUTH_WEST,
    "F": Pipe.SOUTH_EAST,
}


type Coordinates = tuple[int, int]


@dataclass(frozen=True, kw_only=True, slots=True)
class PuzzleInput:
    pipe_map: Mapping[Coordinates, Pipe]
    start_coordinates: Coordinates


def solve(puzzle_input: PuzzleInput) -> int:
    start_coords: Final = puzzle_input.start_coordinates

    steps = 1
    x, y = start_coords
    coords = (x, y-1)
    previous_movement = NORTH
    seen_coordinates = {coords}

    while coords != start_coords:
        steps += 1
        x, y = coords
        node = puzzle_input.pipe_map[coords]
        match node, previous_movement:
            case [Pipe.NORTH_SOUTH, Direction.NORTH]:
                coords = x, y-1
                previous_movement = NORTH
            case [Pipe.NORTH_SOUTH, Direction.SOUTH]:
                coords = x, y+1
                previous_movement = SOUTH
            case [Pipe.EAST_WEST, Direction.EAST]:
                coords = x+1, y
                previous_movement = EAST
            case [Pipe.EAST_WEST, Direction.WEST]:
                coords = x-1, y
                previous_movement = WEST
            case [Pipe.SOUTH_EAST, Direction.NORTH]:
                coords = x+1, y
                previous_movement = EAST
            case [Pipe.SOUTH_EAST, Direction.WEST]:
                coords = x, y+1
                previous_movement = SOUTH
            case [Pipe.NORTH_WEST, Direction.SOUTH]:
                coords = x-1, y
                previous_movement = WEST
            case [Pipe.NORTH_WEST, Direction.EAST]:
                coords = x, y-1
                previous_movement = NORTH
            case [Pipe.SOUTH_WEST, Direction.NORTH]:
                coords = x-1, y
                previous_movement = WEST
            case [Pipe.SOUTH_WEST, Direction.EAST]:
                coords = x, y+1
                previous_movement = SOUTH
            case [Pipe.NORTH_EAST, Direction.WEST]:
                coords = x, y-1
                previous_movement = NORTH
            case [Pipe.NORTH_EAST, Direction.SOUTH]:
                coords = x+1, y
                previous_movement = EAST
            case _:
                assert False, (node, previous_movement)
        assert coords not in seen_coordinates, node
        seen_coordinates.add(coords)

    return steps // 2


def parse_input(filename: str) -> PuzzleInput:
    pipe_map: Final[dict[Coordinates, Pipe]] = {}
    start_coordinates: Coordinates | None = None
    with open(filename) as f:
        for y, line in enumerate(f):
            for x, char in enumerate(line.strip()):
                match char:
                    case "S":
                        pipe = Pipe.NORTH_SOUTH
                        coordinates = x, y
                        start_coordinates = coordinates
                    case ".":
                        continue
                    case _:
                        pipe = INPUT_TO_PIPE[char]
                        coordinates = x, y
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
