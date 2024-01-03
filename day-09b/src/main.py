def find_previous_value(history: list[int]) -> int:
    log = [history]
    latest = history
    while len(set(latest)) != 1:
        latest = [b-a for a, b in zip(latest, latest[1:])]
        log.append(latest)
    log.reverse()
    answer = log[0][0]
    for history in log[1:]:
        answer = history[0] - answer
    return answer


def solve(filename: str) -> int:
    with open(filename) as f:
        lines = f.read().splitlines()
    histories = ([int(string) for string in line.split()] for line in lines)
    return sum(find_previous_value(history) for history in histories)


def main() -> None:
    print(solve("input.txt"))


if __name__ == "__main__":
    main()