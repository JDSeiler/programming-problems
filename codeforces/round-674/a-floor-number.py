import math
t = int(input())

def solve_case():
    n, x = list(map(int, input().split()))
    if n <= 2:
        print(1)
    else:
        n = n - 2
        print(math.ceil(n / x) + 1)

for i in range(0, t):
    solve_case()
