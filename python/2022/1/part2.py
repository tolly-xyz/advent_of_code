def main():
    with open('input.txt', 'r') as f:
        lines = f.readlines()

    elf_calories = 0
    top_three = [0, 0, 0]

    for line in lines:
        if line != '\n':
            elf_calories += int(line)
            continue
        elif elf_calories > top_three[-1]:
            top_three[-1] = elf_calories
            top_three.sort(reverse=True)
        elf_calories = 0

    print(sum(top_three))


if __name__ == '__main__':
    main()