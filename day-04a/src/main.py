from collections.abc import Iterator
from dataclasses import dataclass


@dataclass(frozen=True, slots=True, kw_only=True)
class Card:
    card_id: int
    winning_numbers: frozenset[int]
    numbers_we_have: frozenset[int]

    def total_points(self) -> int:
        intersection = self.winning_numbers & self.numbers_we_have
        if not intersection:
            return 0
        return 2 ** (len(intersection) - 1)


def parse_input(filename: str) -> Iterator[Card]:
    with open(filename) as f:
        for card_id, line in enumerate(f, start=1):
            _, _, data = line.partition(": ")
            left, _, right = data.partition(" | ")
            winning_numbers = frozenset(map(int, left.split()))
            numbers_we_have = frozenset(map(int, right.split()))
            yield Card(
                card_id=card_id,
                winning_numbers=winning_numbers,
                numbers_we_have=numbers_we_have
            )


def solve(filename: str) -> int:
    return sum(card.total_points() for card in parse_input(filename))


def main() -> None:
    print(solve("input.txt"))


if __name__ == "__main__":
    main()