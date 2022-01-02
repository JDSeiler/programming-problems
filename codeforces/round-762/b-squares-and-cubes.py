def solve_case():
    n = int(input())
    largest_square = 1
    i = 2
    numbers = set()
    numbers.add(1)

    while largest_square <= n:
        square = i*i
        cube = square*i

        if square <= n:
            numbers.add(square)
        largest_square = square

        if cube <= n:
            numbers.add(cube)

        i += 1

    print(len(numbers))


t = int(input())
for i in range(t):
    solve_case()
