t = int(input())

def solve_case():
    a, b = list(map(int, input().split()))
    if (a % b) == 0:
        print(0)
    else:
        print(b - (a % b))

for i in range(0, t):
    solve_case()
