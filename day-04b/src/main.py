from collections import Counter
from dataclasses import dataclass
from functools import cache


@dataclass(frozen=True, slots=True, kw_only=True)
class Card:
    card_id: int
    winning_numbers: frozenset[int]
    numbers_we_have: frozenset[int]


@cache
def copied_cards_won(card: Card) -> range:
    num_won = len(card.winning_numbers & card.numbers_we_have)
    return range(card.card_id + 1, card.card_id + 1 + num_won)


def parse_input(filename: str) -> dict[int, Card]:
    cards: dict[int, Card] = {}
    with open(filename) as f:
        for card_id, line in enumerate(f, start=1):
            _, _, data = line.partition(": ")
            left, _, right = data.partition(" | ")
            winning_numbers = frozenset(map(int, left.split()))
            numbers_we_have = frozenset(map(int, right.split()))
            card = Card(
                card_id=card_id,
                winning_numbers=winning_numbers,
                numbers_we_have=numbers_we_have
            )
            cards[card_id] = card
    return cards


def compute_total_scratchcards(cards: dict[int, Card]) -> int:
    counter = Counter(cards.values())
    for card, count in counter.items():
        for card_won_id in copied_cards_won(card):
            counter[cards[card_won_id]] += count
    return counter.total()


def solve(filename: str) -> int:
    cards = parse_input(filename)
    return compute_total_scratchcards(cards)


def main() -> None:
    print(solve("input.txt"))


if __name__ == "__main__":
    main()