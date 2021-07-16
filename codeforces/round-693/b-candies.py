def add_twos(target):
    count = 0
    pile = 0
    while pile < target and pile + 2 <= target:
        pile = pile + 2
        count = count + 1

    return pile
    

def solve_case():
    n = int(input())
    weights = sorted([int(c) for c in input().split()])
    two_amount = sum(list(filter(lambda x: x == 2, weights)))
    one_amount = sum(list(filter(lambda x: x == 1, weights)))
    total = sum(weights)
    # If you can't divide into a whole number, it's obviously impossible
    if total % 2 != 0:
        print("NO")
        return
    # otherwise...
    target = total / 2
    pile = 0
    # Add as many twos as you can
    if two_amount > target:
        pile = add_twos(target)
    else:
        pile = two_amount

    if one_amount >= target-pile:
        print("YES")
    else:
         print("NO")   

t = int(input())
for i in range(0, t):
    solve_case()