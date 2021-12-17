a, b, c = list(map(int, input().split()))

left_space = b - a - 1
right_space = c - b - 1

print(max(right_space, left_space))