def main():
    with open('input.txt', 'r') as f:
        lines = f.readlines()
    
    elf_calories = 0
    max_calories = 0
    for line in lines:
        if line != '\n':
            elf_calories += int(line)
            continue
        elif elf_calories > max_calories:
            max_calories = elf_calories
        elf_calories = 0

    print(max_calories)


if __name__ == '__main__':
    main()