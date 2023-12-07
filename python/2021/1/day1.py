def main():
    file = open("1/input", "r")
    lines = file.readlines()

    part_one(lines)
    part_two(lines)


def part_one(lines):
    count = 0

    for i in range(1, len(lines)):
        line = line_to_num(lines[i])
        prev = line_to_num(lines[i - 1])

        if line > prev:
            count += 1

    print(count)


def part_two(lines):
    count = 0

    for i in range(1, len(lines) - 1):
        if window_sum(lines, i) > window_sum(lines, i - 1):
            count += 1

    print(count)


def window_sum(lines, i):
    n1 = line_to_num(lines[i - 1])
    n2 = line_to_num(lines[i])
    n3 = line_to_num(lines[i + 1])

    return n1 + n2 + n3


def line_to_num(line):
    num_str = line[0:-1]
    return int(num_str)


if __name__ == "__main__":
    main()
