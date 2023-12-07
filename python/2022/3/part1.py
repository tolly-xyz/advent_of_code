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
    for line in lines:
        mid = len(line) // 2
        comp_1, comp_2 = line[:mid], line[mid:-1]
        common = set(comp_1).intersection(comp_2).pop()
        priority_sum += get_item_priority(common)

    print(priority_sum)

if __name__ == '__main__':
    main()