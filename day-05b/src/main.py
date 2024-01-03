from __future__ import annotations

import collections
import dataclasses as dc
import enum
from collections.abc import Iterable, Iterator
from functools import cached_property
from itertools import batched, islice
from operator import attrgetter
from typing import Self

get_range_start = attrgetter("start")
get_range_stop = attrgetter("stop")


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
            source=GardeningThing[source], destination=GardeningThing[destination]
        )


@dc.dataclass(frozen=True)
class InputDataRow:
    destination_start: int
    source_start: int
    range_length: int

    @cached_property
    def source_range(self) -> range:
        return range(self.source_start, self.source_start + self.range_length)

    def convert_single(self, item: int) -> int:
        assert (item in self.source_range) or item == self.source_range.stop
        difference = item - self.source_start
        return self.destination_start + difference

    def convert_range(self, r: range) -> range:
        start = self.convert_single(r.start)
        stop = self.convert_single(r.stop)
        return range(start, stop)


@dc.dataclass(slots=True, kw_only=True, frozen=True)
class InputMap:
    kind: MapKind
    rows: tuple[InputDataRow, ...] = dc.field(repr=False)


def find_range_overlap(x: range, y: range) -> range:
    return range(max(x.start, y.start), min(x.stop, y.stop))


@dc.dataclass(slots=True, kw_only=True, frozen=True)
class RangeMap:
    kind: MapKind
    mapping: dict[range, range]


def sliding_pairs_window(iterable: Iterable[range]) -> Iterator[tuple[range, range]]:
    """Adapted from the itertools sliding_window recipe"""
    # sliding_window('ABCDEFG', 4) --> ABCD BCDE CDEF DEFG
    it = iter(iterable)
    window = collections.deque(islice(it, 1), maxlen=2)
    for x in it:
        window.append(x)
        yield tuple(window)  # type: ignore


def _check_range_mapping_consistency(
    initial: dict[range, range], transformed: dict[range, range]
) -> None:
    """Some debug assertions to check correctness"""
    assert (
        min(initial, key=get_range_start).start
        == min(transformed, key=get_range_start).start
    )
    assert (
        max(initial, key=get_range_stop).stop
        == max(transformed, key=get_range_stop).stop
    )
    assert sum(len(key) for key in initial) == sum(len(key) for key in transformed)
    assert len(transformed) >= len(initial)


def progress_range_pair(
    pair: tuple[range, range], input_map: InputMap
) -> dict[range, range]:
    range_mapping: dict[range, range] = {}
    seed_range, intermediate_range = pair
    assert len(seed_range) == len(intermediate_range)
    for row in input_map.rows:
        overlap = find_range_overlap(intermediate_range, row.source_range)
        if overlap:
            new_key = range(
                seed_range.start + (overlap.start - intermediate_range.start),
                seed_range.stop - (intermediate_range.stop - overlap.stop),
            )
            range_mapping[new_key] = row.convert_range(overlap)

    if not range_mapping:
        return {seed_range: intermediate_range}
    keys = sorted(range_mapping, key=get_range_start)
    first_key, last_key = keys[0], keys[-1]
    if seed_range.start < first_key.start:
        startfill = range(seed_range.start, first_key.start)
        startfill_value = range(
            intermediate_range.start, intermediate_range.start + len(startfill)
        )
        range_mapping[startfill] = startfill_value
    if seed_range.stop > last_key.stop:
        endfill = range(last_key.stop, seed_range.stop)
        endfill_value = range(
            intermediate_range.stop - len(endfill), intermediate_range.stop
        )
        range_mapping[endfill] = endfill_value
    for this_range, next_range in sliding_pairs_window(keys):
        if this_range.stop == next_range.start:
            continue
        in_between = range(this_range.stop, next_range.start)
        in_between_value = range(
            intermediate_range.start + (in_between.start - seed_range.start),
            intermediate_range.stop - (seed_range.stop - in_between.stop),
        )
        range_mapping[in_between] = in_between_value
    _check_range_mapping_consistency(dict([pair]), range_mapping)
    if len(range_mapping) > 1:
        assert any(key != value for key, value in range_mapping.items())
    return range_mapping


def progress_range_map(current_range_map: RangeMap, input_data: InputData) -> RangeMap:
    range_mapping: dict[range, range] = {}
    relevant_input_map = next(
        m
        for m in input_data.maps
        if m.kind.source is current_range_map.kind.destination
    )
    for pair in current_range_map.mapping.items():
        range_mapping |= progress_range_pair(pair, relevant_input_map)
    kind = MapKind(
        source=GardeningThing.seed, destination=relevant_input_map.kind.destination
    )
    _check_range_mapping_consistency(current_range_map.mapping, range_mapping)
    return RangeMap(kind=kind, mapping=range_mapping)


@dc.dataclass(slots=True, kw_only=True, frozen=True)
class InputData:
    seed_ranges: tuple[range, ...]
    maps: tuple[InputMap, ...]


def seedrange_to_locationrange(input_data: InputData) -> RangeMap:
    kind = MapKind(source=GardeningThing.seed, destination=GardeningThing.seed)
    initial_range_map = {r: r for r in input_data.seed_ranges}
    range_map = RangeMap(kind=kind, mapping=initial_range_map)

    while range_map.kind.destination is not GardeningThing.location:
        range_map = progress_range_map(range_map, input_data)

    return range_map


def parse_input(filename: str) -> InputData:
    with open(filename) as f:
        puzzle_input = f.read()
    unparsed_seeds, *unparsed_maps = puzzle_input.split("\n\n")
    pairs = batched(map(int, unparsed_seeds.split(" ")[1:]), 2)
    seed_ranges = tuple(range(a, a + b) for a, b in pairs)
    maps: list[InputMap] = []
    for unparsed_map in unparsed_maps:
        first_line, *rest = unparsed_map.splitlines()
        kind = MapKind.from_description(first_line.partition(" ")[0])
        rows = tuple(InputDataRow(*map(int, line.split())) for line in rest)
        maps.append(InputMap(kind=kind, rows=rows))
    return InputData(seed_ranges=seed_ranges, maps=tuple(maps))


def solve(filename: str) -> int:
    input_data = parse_input(filename)
    range_map = seedrange_to_locationrange(input_data)
    lowest_location_range = min(range_map.mapping.values(), key=get_range_start)
    return lowest_location_range.start


def main() -> None:
    print(solve("input.txt"))


if __name__ == "__main__":
    main()
