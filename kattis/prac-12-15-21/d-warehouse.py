t = int(input())

def solve_case():
    n = int(input())
    toys = {}
    for _i in range(0, n):
        toy, amount = input().split()
        if toy in toys.keys():
            toys[toy] += int(amount)
        else:
            toys[toy] = int(amount)

    toys_as_tuples = [(toy, amount) for toy, amount in toys.items()]

    # The stability of `.sort` means you can do this
    # https://stackoverflow.com/a/6667177
    # But as a trick you need to sort by the secondary key first! 
    # See also: https://docs.python.org/3/howto/sorting.html#sort-stability-and-complex-sorts
    toys_as_tuples.sort(key=lambda entry: entry[0])
    toys_as_tuples.sort(key=lambda entry: entry[1], reverse=True)
    print(len(toys_as_tuples))
    for toy, amount in toys_as_tuples:
        print(f"{toy} {amount}")

for _i in range(0, t):
    solve_case()
