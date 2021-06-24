t = int(input())
for _ in range(0, t):
    n = int(input())
    nums = list(map(int, input().split()))

    A = nums.index(1)
    B = nums.index(n)

    A_left = A+1
    A_right = n-A_left

    B_left = B+1
    B_right = n-B_left

    A_closed = A_left if B < A else A_right + 1
    A_open = n - A_closed + 1

    B_closed = B_left if A < B else B_right + 1
    B_open = n - B_closed + 1

    print(min(A_closed, B_closed, A_open+B_open))