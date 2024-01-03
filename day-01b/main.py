from enum import StrEnum

class Numbers(StrEnum):
    ONE = "one"
    TWO = "two"
    THREE = "three"
    FOUR = "four"
    FIVE = "five"
    SIX = "six"
    SEVEN = "seven"
    EIGHT = "eight"
    NINE = "nine"

    def as_digit(self) -> int:
        return list(Numbers).index(self) + 1


def calculate(filename: str) -> int:
    total = 0
    with open(filename) as f:
        for line in f:
            first: int | None = None
            last: int | None = None

            # find `first`, iterating forwards:
            for i, char in enumerate(line):
                if first is not None:
                    break
                if char.isdigit():
                    first = int(char)
                else:
                    for number in Numbers:
                        if line[i:].startswith(number):
                            first = number.as_digit()
            
            for i in range(len(line)-1, -1, -1):
                if last is not None:
                    break
                char = line[i]
                if char.isdigit():
                    last = int(char)
                else:
                    for number in Numbers:
                        if line[i:].startswith(number):
                            last = number.as_digit()
            
            assert first is not None, "Oops, advent of code is broken!"
            assert last is not None, "Oops, advent of code is broken!"
        
            total += (first * 10) + last
        
    return total


def main() -> None:
    print(calculate("input.txt"))


if __name__ == "__main__":
    main()
