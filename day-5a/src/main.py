import dataclasses as dc
import enum
from collections.abc import Iterator
from functools import cached_property
from typing import Self

class GardeningThing(enum.Enum):
    seed = enum.auto()
    soil = enum.auto()
    fertilizer = enum.auto()
    water = enum.auto()
    light = enum.auto()
    temperature = enum.auto()
    humidity = enum.auto()
    location = enum.auto()


@dc.dataclass(kw_only=True, slots=True, frozen=True)
class MapKind:
    source: GardeningThing
    destination: GardeningThing

    def __repr__(self) -> str:
        return f"MapKind({self.source.name}-to-{self.destination.name})"
    
    @classmethod
    def from_description(cls, description: str) -> Self:
        source, _, destination = description.partition("-to-")
        return cls(
            source=GardeningThing[source],
            destination=GardeningThing[destination]
        )


@dc.dataclass(frozen=True)
class InputDataRow:
    destination_start: int
    source_start: int
    range_length: int

    @cached_property
    def source_range(self) -> range:
        return range(self.source_start, self.source_start + self.range_length)


@dc.dataclass(slots=True, kw_only=True, frozen=True)
class Map:
    kind: MapKind
    rows: tuple[InputDataRow, ...] = dc.field(repr=False)

    def convert(self, item: int) -> int:
        for row in self.rows:
            if item in row.source_range:
                difference = item - row.source_start
                return row.destination_start + difference
        else:
            return item


def location_from_seed(seed: int, maps: tuple[Map, ...]) -> int:
    answer = seed
    thing = GardeningThing.seed
    while thing is not GardeningThing.location:
        relevant_map = next(m for m in maps if m.kind.source is thing)
        answer = relevant_map.convert(answer)
        thing = relevant_map.kind.destination
    return answer


@dc.dataclass(slots=True, kw_only=True, frozen=True)
class InputData:
    seeds: frozenset[int]
    maps: tuple[Map, ...]

    def seed_locations(self) -> Iterator[int]:
        for seed in self.seeds:
            yield location_from_seed(seed, self.maps)


def parse_input(filename: str) -> InputData:
    with open(filename) as f:
        puzzle_input = f.read()
    unparsed_seeds, *unparsed_maps = puzzle_input.split("\n\n")
    seeds = frozenset(map(int, unparsed_seeds.split(" ")[1:]))
    maps: list[Map] = []
    for unparsed_map in unparsed_maps:
        first_line, *rest = unparsed_map.splitlines()
        kind = MapKind.from_description(first_line.partition(" ")[0])
        rows = tuple(InputDataRow(*map(int, line.split())) for line in rest)
        maps.append(Map(kind=kind, rows=rows))
    return InputData(seeds=seeds, maps=tuple(maps))


def solve(filename: str) -> int:
    input_data = parse_input(filename)
    return min(input_data.seed_locations())


def main() -> None:
    print(solve("input.txt"))


if __name__ == "__main__":
    main()
