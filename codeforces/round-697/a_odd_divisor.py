def squeeze(n):
    if n == 1:
        print("NO")
    else:
        if n % 2 == 0:
            squeeze(n/2)
        else:
            print("YES")

cases = int(input())
for i in range(0, cases):
    n = int(input())
    squeeze(n)
