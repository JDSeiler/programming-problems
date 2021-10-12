t = int(input())

def solve_case():
    a, b, c = list(map(int, input().split()))
    diameter = abs(a - b)
    size = diameter*2
    if (a > size) or (b > size) or (c > size):
        print(-1)
    else:
        num = (c+diameter) % size
        if num == 0:
            print(size)
        else:
            print(num)
    
for i in range(0, t):
    solve_case()