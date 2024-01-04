from collections.abc import Iterator, Sequence
from dataclasses import dataclass
from itertools import product, starmap
from operator import attrgetter
from typing import NamedTuple, Self, final


class XYPoint(NamedTuple):
    x: int
    y: int


@final
@dataclass(kw_only=True)
class Brick:
    min_x: int
    max_x: int
    min_y: int
    max_y: int
    min_z: int
    max_z: int

    def __post_init__(self) -> None:
        self._x_range = range(self.min_x, self.max_x + 1)
        self._y_range = range(self.min_y, self.max_y + 1)

    def z_range(self) -> range:
        return range(self.min_z, self.max_z + 1)

    def __repr__(self) -> str:
        return (
            f"Brick("
            f"x=({self.min_x}->{self.max_x}), "
            f"y=({self.min_y}->{self.max_y}), "
            f"z=({self.min_z}->{self.max_z})"
            f")"
        )

    def fall_by_one(self) -> None:
        self.min_z -= 1
        self.max_z -= 1

    def xy_points(self) -> Iterator[XYPoint]:
        return starmap(XYPoint, product(self._x_range, self._y_range))

    @classmethod
    def from_string(cls, string: str) -> Self:
        left, _, right = string.partition("~")
        x0, y0, z0 = map(int, left.split(","))
        x1, y1, z1 = map(int, right.split(","))
        return cls(
            min_x=min(x0, x1),
            max_x=max(x0, x1),
            min_y=min(y0, y1),
            max_y=max(y0, y1),
            min_z=min(z0, z1),
            max_z=max(z0, z1),
        )

    def __hash__(self) -> int:
        return id(self)


type ZCoordinate = int
type GridOfGrids = dict[ZCoordinate, dict[XYPoint, Brick]]


@dataclass(slots=True, frozen=True, kw_only=True)
class PuzzleInput:
    bricks: Sequence[Brick]
    map: GridOfGrids

    @classmethod
    def load(cls, *, input_filename: str) -> Self:
        with open(input_filename) as f:
            bricks = tuple(Brick.from_string(line) for line in f)
        grid_of_grids: GridOfGrids = {}
        for brick in bricks:
            for z in brick.z_range():
                grid = grid_of_grids.setdefault(z, {})
                grid.update(dict.fromkeys(brick.xy_points(), brick))
        return cls(bricks=bricks, map=grid_of_grids)


def drop_brick(brick: Brick, *, map: GridOfGrids) -> None:
    while brick.min_z > 1:
        grid_below = map.setdefault(brick.min_z - 1, {})
        if grid_below and any(point in grid_below for point in brick.xy_points()):
            break
        highest_grid = map[brick.max_z]
        for point in brick.xy_points():
            grid_below[point] = brick
            del highest_grid[point]
        brick.fall_by_one()


def has_two_or_more_bricks_below(
    brick: Brick, *, map: GridOfGrids
) -> bool:
    grid_below = map[brick.min_z - 1]
    bricks_below: set[Brick] = set()
    for point in brick.xy_points():
        brick_below: Brick | None = grid_below.get(point)
        if brick_below is not None:
            bricks_below.add(brick_below)
        if len(bricks_below) > 1:
            return True
    return False


def brick_could_safely_be_disintegrated(brick: Brick, *, map: GridOfGrids) -> bool:
    grid_above = map.setdefault(brick.max_z + 1, {})
    if not grid_above:
        return True
    bricks_above: set[Brick] = set()
    for point in brick.xy_points():
        brick_above: Brick | None = grid_above.get(point)
        if brick_above is not None:
            bricks_above.add(brick_above)
    return all(
        has_two_or_more_bricks_below(brick_above, map=map)
        for brick_above in bricks_above
    )


def solve(input_filename: str) -> int:
    puzzle_input = PuzzleInput.load(input_filename=input_filename)
    for brick in sorted(puzzle_input.bricks, key=attrgetter("min_z")):
        drop_brick(brick, map=puzzle_input.map)
    return sum(
        1
        for brick in puzzle_input.bricks
        if brick_could_safely_be_disintegrated(brick, map=puzzle_input.map)
    )


def main() -> None:
    print(solve("input.txt"))


if __name__ == "__main__":
    main()
