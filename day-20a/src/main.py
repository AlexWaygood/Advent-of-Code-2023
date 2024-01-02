from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import ClassVar, NamedTuple, Protocol


class PulseKind(Enum):
    HIGH = auto()
    LOW = auto()


class PulseRequest(NamedTuple):
    kind: PulseKind
    sender: Module


class Module(Protocol):
    @property
    def name(self) -> str: ...
    @property
    def connections(self) -> list[Module]: ...
    def receive_pulse(
        self, *, kind: PulseKind, from_: Module
    ) -> PulseRequest | None: ...


@dataclass(slots=True, kw_only=True)
class FlipFlopModule:
    name: str
    connections: list[Module] = field(init=False, default_factory=list)
    is_on: bool = field(init=False, default=False)

    def receive_pulse(self, *, kind: PulseKind, from_: Module) -> PulseRequest | None:
        match self.is_on, kind:
            case (_, PulseKind.HIGH):
                return None
            case (True, PulseKind.LOW):
                self.is_on = False
                return PulseRequest(kind=PulseKind.LOW, sender=self)
            case (False, PulseKind.LOW):
                self.is_on = True
                return PulseRequest(kind=PulseKind.HIGH, sender=self)


@dataclass(slots=True, kw_only=True, frozen=True)
class ConjunctionModule:
    name: str
    connections: list[Module] = field(init=False, default_factory=list)
    memory: dict[str, PulseKind] = field(init=False, default_factory=dict)

    def receive_pulse(self, *, kind: PulseKind, from_: Module) -> PulseRequest:
        assert from_.name in self.memory
        self.memory[from_.name] = kind
        if all(kind is PulseKind.HIGH for kind in self.memory.values()):
            to_send = PulseKind.LOW
        else:
            to_send = PulseKind.HIGH
        return PulseRequest(kind=to_send, sender=self)


@dataclass(slots=True, kw_only=True, frozen=True)
class BroadcastModule:
    name: ClassVar = "broadcaster"
    connections: list[Module] = field(init=False, default_factory=list)

    def receive_pulse(
        self, *, kind: PulseKind, from_: Module | None = None
    ) -> PulseRequest:
        return PulseRequest(kind=kind, sender=self)


@dataclass(slots=True, frozen=True, kw_only=True)
class UntypedModule:
    name: str
    connections: ClassVar = []

    def receive_pulse(self, *, kind: PulseKind, from_: Module) -> None:
        return None


@dataclass(slots=True, frozen=True, kw_only=True)
class PulseStatistics:
    high_pulses_sent: int = 0
    low_pulses_sent: int = 0

    def __add__(self, other: PulseStatistics) -> PulseStatistics:
        if not isinstance(other, PulseStatistics):  # pyright: ignore[reportUnnecessaryIsInstance]
            return NotImplemented
        return PulseStatistics(
            high_pulses_sent=(self.high_pulses_sent + other.high_pulses_sent),
            low_pulses_sent=(self.low_pulses_sent + other.low_pulses_sent),
        )

    def multiply(self) -> int:
        return self.high_pulses_sent * self.low_pulses_sent


def push_button(broadcaster: BroadcastModule) -> PulseStatistics:
    pulse_requests = deque([broadcaster.receive_pulse(kind=PulseKind.LOW)])
    high_pulses_sent = 0
    low_pulses_sent = 1

    while pulse_requests:
        request = pulse_requests.popleft()
        for connection in request.sender.connections:
            match request.kind:
                case PulseKind.LOW:
                    low_pulses_sent += 1
                case PulseKind.HIGH:
                    high_pulses_sent += 1
            new_request = connection.receive_pulse(
                kind=request.kind, from_=request.sender
            )
            if new_request is not None:
                pulse_requests.append(new_request)

    return PulseStatistics(
        high_pulses_sent=high_pulses_sent, low_pulses_sent=low_pulses_sent
    )


def solve(broadcaster: BroadcastModule) -> int:
    cumulative_stats = sum(
        (push_button(broadcaster) for _ in range(1000)), start=PulseStatistics()
    )
    return cumulative_stats.multiply()


@dataclass(frozen=True, kw_only=True, slots=True)
class LineInfo:
    left: str
    right: list[str]


def parse_input(input_lines: list[str]) -> BroadcastModule:
    lines: list[LineInfo] = []
    for line in input_lines:
        left, _, right = line.strip().partition(" -> ")
        lines.append(LineInfo(left=left, right=right.split(", ")))

    modules: dict[str, Module] = {}
    broadcaster = BroadcastModule()
    for line in lines:
        if line.left == "broadcaster":
            continue
        name = line.left[1:]
        match char := line.left[0]:
            case "%":
                modules[name] = FlipFlopModule(name=name)
            case "&":
                modules[name] = ConjunctionModule(name=name)
            case _:
                assert False, f"Unexpected first character {char}"

    for line in lines:
        these_modules: list[Module] = [
            modules.get(name, UntypedModule(name=name)) for name in line.right
        ]
        if line.left == "broadcaster":
            broadcaster.connections.extend(these_modules)
            continue
        module = modules[line.left[1:]]
        module.connections.extend(these_modules)

    for module in modules.values():
        if isinstance(module, ConjunctionModule):
            module.memory.update({
                input.name: PulseKind.LOW for input in modules.values()
                if module in input.connections
            })

    return broadcaster


def main() -> None:
    with open("input.txt") as f:
        input_lines = list(f)
    broadcaster = parse_input(input_lines)
    print(solve(broadcaster))


if __name__ == "__main__":
    main()


def test_example_one() -> None:
    example = """\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"""
    broadcaster = parse_input(example.splitlines())  # pyright: ignore
    stats = [push_button(broadcaster) for _ in range(1000)]
    for stat in stats:
        assert stat.high_pulses_sent == 4
        assert stat.low_pulses_sent == 8
    answer = sum(stats, start=PulseStatistics()).multiply()
    assert answer == 32000000
