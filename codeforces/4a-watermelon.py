def solve_case(w):
    if w % 2 == 0 and w != 2:
        print('YES')
    else:
        print('NO')

weight = int(input())
solve_case(weight)
