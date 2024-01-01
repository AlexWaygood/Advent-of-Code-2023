from __future__ import annotations

from collections.abc import Callable, Mapping, Sequence
from dataclasses import dataclass
from enum import Enum
from typing import NamedTuple, Self


class DecisionKind(Enum):
    ACCEPT = "A"
    REJECT = "R"
    OTHER_WORKFLOW = None


class Decision(NamedTuple):
    kind: DecisionKind
    data: str | None = None

    @classmethod
    def from_string(cls, s: str) -> Self:
        match s:
            case "A" | "R":
                return cls(DecisionKind(s))
            case _:
                return cls(DecisionKind.OTHER_WORKFLOW, data=s)


@dataclass(slots=True)
class Part:
    x: int
    m: int
    a: int
    s: int

    def score(self) -> int:
        return self.x + self.m + self.a + self.s

    @classmethod
    def from_string(cls, s: str) -> Self:
        part = cls(0, 0, 0, 0)
        for section in s[1:-1].split(","):
            attr, _, value_str = section.partition("=")
            setattr(part, attr, int(value_str))
        return part


def rule_from_string(s: str) -> Callable[[Part], Decision | None]:
    match list(s):
        case ['x' | 'm' | 'a' | 's' as attr, '>' | '<' as cmp, *rest]:
            digits, _, outcome = "".join(rest).partition(":")
            value = int(digits)
            decision = Decision.from_string(outcome)
            match cmp:
                case ">":
                    return lambda p: decision if getattr(p, attr) > value else None
                case "<":
                    return lambda p: decision if getattr(p, attr) < value else None
        case _:
            decision = Decision.from_string(s)
            return lambda _: decision


@dataclass(slots=True, frozen=True, kw_only=True)
class Workflow:
    name: str
    rules: Sequence[Callable[[Part], Decision | None]]

    @classmethod
    def from_string(cls, s: str) -> Self:
        name, _, rule_strings = s.strip()[:-1].partition("{")
        rules = tuple(
            rule_from_string(rule_string) for rule_string in rule_strings.split(",")
        )
        return cls(name=name, rules=rules)

    def process(self, part: Part) -> Decision:
        for rule in self.rules:
            ruling = rule(part)
            if ruling is not None:
                return ruling
        assert False, (
            "At least one rule in self.rules should have returned a `Decision`!"
        )

    def __str__(self) -> str:
        return f"Workflow({self.name!r}, <{len(self.rules)} rules>)"


@dataclass(slots=True, frozen=True, kw_only=True)
class PuzzleInput:
    workflow_map: Mapping[str, Workflow]
    parts: Sequence[Part]

    @classmethod
    def from_string(cls, s: str) -> Self:
        workflow_strings, _, part_strings = s.partition("\n\n")
        workflows = (
            Workflow.from_string(line) for line in workflow_strings.split("\n")
        )
        workflow_map = {w.name: w for w in workflows}
        parts = tuple(Part.from_string(line) for line in part_strings.split("\n"))
        return cls(workflow_map=workflow_map, parts=parts)


def solve(filename: str) -> int:
    with open(filename) as f:
        puzzle_input = PuzzleInput.from_string(f.read())
    answer = 0
    for part in puzzle_input.parts:
        outcome = Decision.from_string("in")
        while True:
            match outcome.kind:
                case DecisionKind.ACCEPT:
                    answer += part.score()
                    break
                case DecisionKind.REJECT:
                    break
                case DecisionKind.OTHER_WORKFLOW:
                    assert outcome.data is not None
                    workflow = puzzle_input.workflow_map[outcome.data]
                    outcome = workflow.process(part)
    return answer


def main() -> None:
    print(solve("input.txt"))


if __name__ == "__main__":
    main()
