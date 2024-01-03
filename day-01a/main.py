import sys

def calculate(filename: str) -> int:
    total = 0
    with open(filename) as f:
        for line in f:
            first: int | None = None
            last: int | None = None
            for char in line:
                if char.isdigit():
                    if first is None:
                        first = int(char)
                    last = int(char)
            assert first is not None, "Oops, Advent of Code is broken!"
            assert last is not None, "Oops, Advent of Code is broken!"
            total += (first * 10) + last
    return total


def main(filename: str) -> None:
    print(calculate(filename))


if __name__ == "__main__":
    main(sys.argv[1])
