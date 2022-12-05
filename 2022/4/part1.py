def main():
    with open('2022/day_04/input.txt') as f:
        lines = f.read().split()
    
    count = 0
    for line in lines:
        a, b = line.split(',')
        a_lo, a_up = map(int, a.split('-'))
        b_lo, b_up = map(int, b.split('-'))
        
        bounds_check = (a_lo - b_lo) * (a_up - b_up)
        if bounds_check <= 0:
            count += 1
    
    print(count)


if __name__ == '__main__':
    main()