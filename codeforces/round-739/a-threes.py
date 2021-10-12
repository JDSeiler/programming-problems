t = int(input())

good = []

def generate_list():
    # pad with 0 so that I can be really lazy later
    good.append(0)
    for i in range(0, 10_000):
        if not(i % 3 == 0) and not(str(i)[-1] == '3'):
            good.append(i)

def solve_case():
    k = int(input())
    print(good[k])


generate_list()
for i in range(0, t):
    solve_case()
