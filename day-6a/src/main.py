import operator
from collections.abc import Iterator
from dataclasses import dataclass
from functools import reduce


@dataclass(slots=True, frozen=True, kw_only=True)
class HypotheticalRaceAttempt:
    time_held_down: int
    available_time: int
    record_distance: int

    def beats_record(self) -> bool:
        speed = self.time_held_down
        remaining_time = self.available_time - self.time_held_down
        distance_travelled = speed * remaining_time
        return distance_travelled > self.record_distance


@dataclass(slots=True, frozen=True, kw_only=True)
class ScheduledRace:
    available_time: int
    record_distance: int

    def ways_to_win(self) -> int:
        total = 0
        middle_reached = False
        for time_held_down in reversed(range(1, self.available_time)):
            hypothetical_attempt = HypotheticalRaceAttempt(
                time_held_down=time_held_down,
                available_time=self.available_time,
                record_distance=self.record_distance
            )
            if hypothetical_attempt.beats_record():
                total += 1
                middle_reached = True
            else:
                if middle_reached:
                    break
        return total


def parse_input(filename: str) -> Iterator[ScheduledRace]:
    with open(filename) as f:
        first_line, second_line = f.read().splitlines()
    times = map(int, first_line.split()[1:])
    distances = map(int, second_line.split()[1:])
    for time, distance in zip(times, distances):
        yield ScheduledRace(available_time=time, record_distance=distance)


def solve(filename: str) -> int:
    scheduled_races = parse_input(filename)
    return reduce(operator.mul, (race.ways_to_win() for race in scheduled_races))


def main() -> None:
    print(solve("input.txt"))


if __name__ == "__main__":
    main()
