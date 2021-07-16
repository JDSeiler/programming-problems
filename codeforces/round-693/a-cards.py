def solve_case():
    w,h,n = [int(c) for c in input().split()]

    w_sheets = 2**num_divisions(w) if w % 2 == 0 else 1
    h_sheets = 2**num_divisions(h) if h % 2 == 0 else 1

    total = w_sheets * h_sheets
    if total >= n:
        print("YES")
    else:
        print("NO")

def num_divisions(n):
    count = 0;
    while n % 2 == 0:
        count = count + 1
        n = n / 2
    
    return count;


t = int(input())
for i in range(0, t):
    solve_case()