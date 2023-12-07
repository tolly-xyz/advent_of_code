def main():
    file = open("2/input", "r")
    lines = file.readlines()

    part_one(lines)
    part_two(lines)



def part_one(lines):
    x = 0
    y = 0

    for line in lines:
        command = read_command(line)
        direction = command[0]
        distance = int(command[1])

        if direction == "forward":
            x += distance
        elif direction == "up":
            y -= distance
        elif direction == "down":
            y += distance
    
    print("part 1: ", x, y, x * y)


def part_two(lines):
    x = 0
    y = 0
    aim = 0

    for line in lines:
        command = read_command(line)
        direction = command[0]
        amount = int(command[1])

        if direction == "forward":
            x += amount
            y += amount * aim
        elif direction == "up":
            aim -= amount
        elif direction == "down":
            aim += amount
    
    print("part 2: ", x, y, x * y)


def read_command(line):
    line = line[:-1] # trim off \n
    command = line.split(" ")
    return command


if __name__ == "__main__":
    main()