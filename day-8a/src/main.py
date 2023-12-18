from __future__ import annotations

from collections.abc import Sequence, Mapping
from dataclasses import dataclass
from enum import Enum
from itertools import cycle
from typing import NewType

class StepKind(Enum):
    RIGHT = "R"
    LEFT = "L"


type StepSequence = Sequence[StepKind]
Place = NewType("Place", str)

@dataclass(frozen=True, slots=True, kw_only=True)
class Node:
    place: Place
    leftwards: Place
    rightwards: Place


type NodeMap = Mapping[Place, Node]


def move(
    *, from_: Node, direction: StepKind, node_map: NodeMap
) -> Node:
    new_place = from_.leftwards if direction is StepKind.LEFT else from_.rightwards
    return node_map[new_place]


def compute_steps_needed(puzzle_input: PuzzleInput) -> int:
    node = puzzle_input.node_map[Place("AAA")]
    steps_taken = 0
    direction_iter = cycle(puzzle_input.step_sequence)
    while node.place != Place("ZZZ"):
        node = move(
            from_=node, direction=next(direction_iter), node_map=puzzle_input.node_map
        )
        steps_taken += 1
    return steps_taken


@dataclass(frozen=True, slots=True, kw_only=True)
class PuzzleInput:
    step_sequence: StepSequence
    node_map: NodeMap


def parse_input(filename: str) -> PuzzleInput:
    with open(filename) as f:
        unparsed_input = f.read()
    first_line, rest = unparsed_input.split("\n\n")
    steps = [StepKind(char) for char in first_line]
    node_map: dict[Place, Node] = {}
    for line in rest.splitlines():
        place, _, rest = line.partition(" = ")
        left, _, right = rest.strip("()").partition(", ")
        node_map[Place(place)] = Node(
            place=Place(place), leftwards=Place(left), rightwards=Place(right)
        )
    return PuzzleInput(step_sequence=steps, node_map=node_map)


def solve(filename: str) -> int:
    puzzle_input = parse_input(filename)
    return compute_steps_needed(puzzle_input)


def main() -> None:
    print(solve("input.txt"))


if __name__ == "__main__":
    main()
