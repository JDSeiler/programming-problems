import sys
lines = sys.stdin.readlines()
for line in lines:
    n = int(line.strip())
    if n == 1 or n == 2:
        print(n)
    else:
        print(n + n-2)
