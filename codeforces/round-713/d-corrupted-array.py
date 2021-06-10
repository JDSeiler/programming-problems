t = int(input())

def solve_case():
    nums = list(map(lambda v: int(v), input().split()))
    L = max(nums)
    nums.remove(L)
    R = nums.copy()
    sum_r = sum(R)
    H = sum_r / 2

    sums = [(sum_r - r_i, r_i) for r_i in R]

    for s, r_i in sums:
        if s == L or s == H:
            R.remove(r_i)
            return R
        else:
            continue

    return -1         

for c in range(0, t):
    n = int(input())
    result = solve_case()
    if result == -1:
        print(-1)
    else:
        for i in result:
            print(str(i) + " ", end="")
        print()