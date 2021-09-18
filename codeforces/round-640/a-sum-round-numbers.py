t = int(input())

def solve_case():
    parts = []
    number = input()

    for (idx, d) in enumerate(number):
        if d == '0':
            continue
        else:
            place_val = len(number) - idx
            part = d + '0' * (place_val-1)
            parts.append(part)

    print(len(parts))
    for part in parts:
        print(part, end=" ")
    print()

for i in range(0, t):
    solve_case()
