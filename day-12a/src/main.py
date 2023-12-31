import re
from collections.abc import Iterator
from contextlib import suppress
from dataclasses import dataclass
from enum import Enum
from functools import cache
from itertools import groupby
from typing import Self

import pytest


class Condition(Enum):
    DAMAGED = "#"
    UNKNOWN = "?"
    OPERATIONAL = "."

    @cache
    def is_operational(self) -> bool:
        return self is Condition.OPERATIONAL

    def __repr__(self) -> str:
        return self.name[0]


def find_conditions(string: str) -> Iterator[Condition]:
    for char in re.sub(r"\.+", ".", string).strip("."):
        yield Condition(char)


@cache
def num_possible_fits(
    contiguous_broken: tuple[int, ...], conditions: tuple[Condition, ...]
) -> int:
    if len(conditions) < len(contiguous_broken):
        return 0

    if conditions[0] is Condition.OPERATIONAL:
        return num_possible_fits(contiguous_broken, conditions[1:])

    grouped_by_operational = [
        (operational, sum(1 for _ in group_iter))
        for operational, group_iter in groupby(conditions, key=Condition.is_operational)
    ]
    assert not grouped_by_operational[0][0]
    assert not grouped_by_operational[-1][0]

    if sum(contiguous_broken) > sum(
        group_length
        for operational, group_length in grouped_by_operational
        if not operational
    ):
        return 0

    grouped_by_condition = [
        (condition, sum(1 for _ in group_iter))
        for condition, group_iter in groupby(conditions)
    ]
    assert grouped_by_condition[0][0] is not Condition.OPERATIONAL
    assert grouped_by_condition[-1][0] is not Condition.OPERATIONAL

    first_contiguous = contiguous_broken[0]

    # If the first group of (Damaged + Unknown) doesn't fit the first contiguous group:
    if grouped_by_operational[0][1] < first_contiguous:
        first_operational_index = grouped_by_operational[0][1] + 1
        if any(
            condition is Condition.DAMAGED
            for condition in conditions[:first_operational_index]
        ):
            return 0
        return num_possible_fits(
            contiguous_broken, conditions[first_operational_index:]
        )

    # If the last group of (Damaged + Unknown) doesn't fit the last contiguous group:
    if grouped_by_operational[-1][1] < contiguous_broken[-1]:
        last_operational_index = len(conditions) - grouped_by_operational[-1][1] - 1
        if any(
            condition is Condition.DAMAGED
            for condition in conditions[last_operational_index:]
        ):
            return 0
        return num_possible_fits(contiguous_broken, conditions[:last_operational_index])

    answer = 0

    if len(contiguous_broken) == 1:
        if any(condition is Condition.DAMAGED for condition, _ in grouped_by_condition):
            for i in range(len(conditions)):
                if i != 0 and conditions[i - 1] is Condition.DAMAGED:
                    break
                if any(
                    condition is Condition.DAMAGED
                    for condition in conditions[(i + first_contiguous) :]
                ):
                    continue
                to_test = conditions[i : (i + first_contiguous)]
                if len(to_test) < first_contiguous:
                    break
                if any(condition is Condition.OPERATIONAL for condition in to_test):
                    continue
                if not any(condition is Condition.DAMAGED for condition in to_test):
                    continue
                answer += 1
        else:
            for condition, group_length in grouped_by_condition:
                if (
                    condition is Condition.UNKNOWN
                    and group_length >= first_contiguous
                ):
                    answer += (group_length - first_contiguous) + 1
    else:
        # We now have a sequence of >1 contiguous groups, where:
        # (1) the first group of (Damaged + Unknown)
        #     is at least big enough to fit the first contiguous group, and;
        # (2) the last group of (Damaged + Unknown)
        #     is at least big enough to fit the last contiguous group
        range_to_test = (grouped_by_operational[0][1] - first_contiguous) + 1
        for i in range(range_to_test):
            if i != 0 and conditions[i - 1] is Condition.DAMAGED:
                break
            with suppress(IndexError):
                if conditions[i + first_contiguous] is Condition.DAMAGED:
                    continue
            remaining = conditions[(i + first_contiguous + 1) :]
            answer += num_possible_fits(contiguous_broken[1:], remaining)

        if all(
            condition is Condition.UNKNOWN for condition in conditions[:range_to_test]
        ):
            answer += num_possible_fits(contiguous_broken, conditions[range_to_test:])

    return answer


@dataclass(slots=True, kw_only=True, frozen=True)
class Row:
    conditions: tuple[Condition, ...]
    contiguous_broken_groups: tuple[int, ...]

    @classmethod
    def from_string(cls, line: str) -> Self:
        left, _, right = line.partition(" ")
        conditions = tuple(find_conditions(left))
        contiguous_broken_groups = tuple(int(val) for val in right.split(","))
        return cls(
            conditions=conditions, contiguous_broken_groups=contiguous_broken_groups
        )

    def num_possible_arrangements(self) -> int:
        return num_possible_fits(self.contiguous_broken_groups, self.conditions)


def parse_input(filename: str) -> Iterator[Row]:
    with open(filename) as f:
        for line in f:
            yield Row.from_string(line)


def solve(filename: str) -> int:
    return sum(row.num_possible_arrangements() for row in parse_input(filename))


def main() -> None:
    print(solve("input.txt"))


if __name__ == "__main__":
    main()


@pytest.mark.parametrize(
    ("input", "solution"),
    [
        ("???.### 1,1,3", 1),
        (".??..??...?##. 1,1,3", 4),
        ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
        ("????.#...#... 4,1,1", 1),
        ("????.######..#####. 1,6,5", 4),
        ("?###???????? 3,2,1", 10),
        ("?????#?#?.?#?#. 7,3", 2),
        ("#??#?#.?.?????? 6,1,5", 2),
        ("??#??????#???.? 4,3", 9),
        ("?.?..?..????? 1,4", 6),
        ("?#??###?#????#?. 6,2,3", 2),
        ("???..????# 1,3,1", 3),
        ("?#??????#??. 4,2", 4),
        ("??.?????#??#? 2,1,5", 8),
        (".?#?.###??. 1,5", 1),
        ("?.???#?##?? 1,7", 4),
        ("?#???..?.#? 2,2,1", 1),
        ("????##???#?.??. 5,2,1", 10),
        ("????????.?##???????? 3,1,1,4,1,1", 64),
        ("???.???#???? 1,1,3,1", 18),
        ("??????.#.#? 6,1,1", 1),
        ("????...????#??# 1,1,1,3,1", 9),
        ("???#??#?#?###.??? 12,1", 3),
        ("??#??#????#? 5,3", 4),
        ("???#?##??????.???.?? 1,8,1,1", 48),
    ],
)
def test_examples(input: str, solution: int) -> None:
    row = Row.from_string(input)
    assert row.num_possible_arrangements() == solution
