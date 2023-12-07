AGAINST_ROCK = {
    'win': 'paper',
    'lose': 'scissors',
    'draw': 'rock'
}

AGAINST_PAPER = {
    'win': 'scissors',
    'lose': 'rock',
    'draw': 'paper'
}

AGAINST_SCISSORS = {
    'win': 'rock',
    'lose': 'paper',
    'draw': 'scissors'
}

OPP_CODES = {
    'A': AGAINST_ROCK,
    'B': AGAINST_PAPER,
    'C': AGAINST_SCISSORS
}

MY_CODES = {
    'X': 'lose',
    'Y': 'draw',
    'Z': 'win'
}

POINT_MAPPING = {
    'rock': 1,
    'paper': 2,
    'scissors': 3,
    'lose': 0,
    'draw': 3,
    'win': 6
}

def get_round_score(opp_code, my_code):
    shape_against = OPP_CODES[opp_code]
    target_outcome = MY_CODES[my_code]

    my_shape = shape_against[target_outcome]
    score = POINT_MAPPING[target_outcome] + POINT_MAPPING[my_shape]

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