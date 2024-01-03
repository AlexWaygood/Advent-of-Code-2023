def find_next_value(history: list[int]) -> int:
    latest = history
    answer = history[-1]
    while len(set(latest)) != 1:
        latest = [b-a for a, b in zip(latest, latest[1:])]
        answer += latest[-1]
    return answer


def solve(filename: str) -> int:
    with open(filename) as f:
        lines = f.read().splitlines()
    histories = ([int(string) for string in line.split()] for line in lines)
    return sum(find_next_value(history) for history in histories)


def main() -> None:
    print(solve("input.txt"))


if __name__ == "__main__":
    main()
