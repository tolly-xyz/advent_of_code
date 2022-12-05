def get_item_priority(item: str):
    if item.islower():
        return ord(item) - 96
    elif item.isupper():
        return ord(item) - 38
    else:
        raise


def main():
    with open('2022/day_3/input.txt', 'r') as f:
        lines = f.readlines()
    
    priority_sum = 0
    for i in range(0, len(lines), 3):
        line_1, line_2, line_3 = lines[i:i+3]
        common = set(line_1[:-1]).intersection(line_2[:-1]).intersection(line_3[:-1]).pop()
        priority_sum += get_item_priority(common)

    print(priority_sum)

if __name__ == '__main__':
    main()