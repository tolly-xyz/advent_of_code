from collections import defaultdict, deque
from pprint import pprint

STACKS = defaultdict(deque)

def parse_row(line: str) -> str:
    return line[1::4]


def parse_crate_stacks(line: str) -> None:
    level_crates = parse_row(line)
    for i, crate in enumerate(level_crates, start=1):
        if crate != ' ':
            STACKS[i].appendleft(crate)


def parse_step(line: str) -> None:
    if not line:
        return

    count, remove, add = [int(num) for num in line.split() if num.isnumeric()]
    for _ in range(count):
        STACKS[add].append(STACKS[remove].pop())


def main():
    with open('input.txt') as f:
        lines = f.read().split('\n')

    line = ''
    i = 0

    for i, line in enumerate(lines):
        if parse_row(line).isnumeric():
            break
        else:
            parse_crate_stacks(line)

    for line in lines[i+2:]:
        parse_step(line)

    print(''.join([stack[-1] for _, stack in sorted(STACKS.items())]))
    

if __name__ == '__main__':
    main()
