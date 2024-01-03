from __future__ import annotations

from collections import Counter
from collections.abc import Iterable, Iterator
from dataclasses import dataclass
from enum import Enum
from functools import cache, total_ordering
from typing import Self


@total_ordering
class OrderedEnum(Enum):
    def __lt__(self, other: Self) -> bool:
        if type(other) is not type(self):
            return NotImplemented
        return self.value < other.value


class Card(OrderedEnum):
    TWO = 2
    THREE = 3
    FOUR = 4
    FIVE = 5
    SIX = 6
    SEVEN = 7
    EIGHT = 8
    NINE = 9
    TEN = 10
    J = 11
    Q = 12
    K = 13
    A = 14

    def __repr__(self) -> str:
        if self.value < 11:
            return f"Card({self.value})"
        return f"Card({self.name})"


class HandCategory(OrderedEnum):
    HIGH_CARD = 0
    ONE_PAIR = 1
    TWO_PAIR = 2
    THREE_OF_A_KIND = 3
    FULL_HOUSE = 4
    FOUR_OF_A_KIND = 5
    FIVE_OF_A_KIND = 6


@cache
def determine_hand_category(card_counts: tuple[int, ...]) -> HandCategory:
    match card_counts:
        case [5]:
            return HandCategory.FIVE_OF_A_KIND
        case [4, 1]:
            return HandCategory.FOUR_OF_A_KIND
        case [3, 2]:
            return HandCategory.FULL_HOUSE
        case [3, 1, 1]:
            return HandCategory.THREE_OF_A_KIND
        case [2, 2, 1]:
            return HandCategory.TWO_PAIR
        case [2, *_]:
            return HandCategory.ONE_PAIR
        case _:
            return HandCategory.HIGH_CARD


@total_ordering
@dataclass(frozen=True, kw_only=True, slots=True)
class Hand:
    cards: tuple[Card, ...]
    bid: int

    def category(self) -> HandCategory:
        return determine_hand_category(
            tuple(sorted(Counter(self.cards).values(), reverse=True))
        )

    def __lt__(self, other: Hand) -> bool:
        our_category, other_category = self.category(), other.category()
        if our_category is not other_category:
            return our_category < other_category
        return self.cards < other.cards


def winnings_of_hand(hand: Hand, *, rank: int) -> int:
    return hand.bid * rank


def total_winnings(hands: Iterable[Hand]) -> int:
    return sum(
        winnings_of_hand(hand, rank=index)
        for index, hand in enumerate(sorted(hands), start=1)
    )


def parse_input(filename: str) -> Iterator[Hand]:
    with open(filename) as f:
        for line in f:
            unparsed_hand, unparsed_bid = line.split()
            cards: list[Card] = []
            for char in unparsed_hand:
                if char in "JQKA":
                    cards.append(Card[char])
                elif char == "T":
                    cards.append(Card.TEN)
                else:
                    cards.append(Card(int(char)))
            yield Hand(cards=tuple(cards), bid=int(unparsed_bid))


def solve(filename: str) -> int:
    hands = parse_input(filename)
    return total_winnings(hands)


def main() -> None:
    print(solve("input.txt"))


if __name__ == "__main__":
    main()
