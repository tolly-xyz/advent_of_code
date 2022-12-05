SHAPE_CODES = {
    'A': 'rock',
    'B': 'paper',
    'C': 'scissors',
    'X': 'rock',
    'Y': 'paper',
    'Z': 'scissors',
}


POINT_MAPPING = {
    'rock': 1,
    'paper': 2,
    'scissors': 3
}


def get_winning_shape(opp_shape, my_shape):
    if opp_shape == my_shape:
        return None

    all_shapes = {'rock', 'paper', 'scissors'}
    thrown_shapes = {opp_shape, my_shape}
    leftover = all_shapes - thrown_shapes

    match leftover.pop():
        case 'rock':
            return 'scissors'
        case 'paper':
            return 'rock'
        case 'scissors':
            return'paper'
        case _:
            raise


def get_round_score(opp_code, my_code):
    my_shape = SHAPE_CODES[my_code]
    score = POINT_MAPPING[my_shape]

    opp_shape = SHAPE_CODES[opp_code]
    winning_shape = get_winning_shape(opp_shape, my_shape)
    
    if my_shape == winning_shape:
        return score + 6
    elif winning_shape is None:
        return score + 3
    else:
        return score


def main():
    with open('2022/day_2/input_1.txt', 'r') as f:
        lines = f.readlines()
    
    score = 0
    for line in lines:
        opp_code, my_code = line.strip().split(' ')
        score += get_round_score(opp_code, my_code)    
    
    print(score)


if __name__ == '__main__':
    main()
