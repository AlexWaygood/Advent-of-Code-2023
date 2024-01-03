import re
from collections.abc import Iterator


def gather_surrounding_chars(
    loc_range: range,
    *,
    lineno: int,
    line: str,
    all_lines: list[str]
) -> Iterator[str]:
    left = max(0, loc_range.start - 1)
    right = min(len(all_lines[0]) - 1, loc_range.stop)

    try:
        prev_line = all_lines[lineno - 1]
    except IndexError:
        pass
    else:
        yield from prev_line[left:(right+1)]

    try:
        next_line = all_lines[lineno + 1]
    except IndexError:
        pass
    else:
        yield from next_line[left:(right+1)]

    if not line[left].isdigit():
        yield line[left]
    if not line[right].isdigit():
        yield line[right]


def char_is_symbol(char: str) -> bool:
    return char != "." and not char.isdigit()


def is_part_number(
    loc_range: range, lineno: int, line: str, all_lines: list[str]
) -> bool:
    surrounding_chars = gather_surrounding_chars(
        loc_range, lineno=lineno, line=line, all_lines=all_lines
    )
    return any(char_is_symbol(char)for char in surrounding_chars)


def gather_part_numbers_from_line(
    lineno: int, line: str, all_lines: list[str]
) -> Iterator[int]:
    for match in re.finditer(r"\d+", line):
        loc_range = range(match.start(), match.end())
        if is_part_number(loc_range, lineno, line, all_lines):
            yield int(match[0])


def gather_part_numbers_from_file(lines: list[str]) -> Iterator[int]:
    for i, line in enumerate(lines):
        yield from gather_part_numbers_from_line(i, line, lines)


def solve(filename: str) -> int:
    with open(filename) as f:
        lines = f.read().splitlines()
    return sum(gather_part_numbers_from_file(lines))


def main() -> None:
    print(solve("input.txt"))


if __name__ == "__main__":
    main()
