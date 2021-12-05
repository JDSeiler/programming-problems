import math

t = int(input())

def generate_factors(x):
    factors = []
    # based on
    # https://stackoverflow.com/a/6800214

    # x is at most 10^12, but the bound is sqrt(x) so it's ok
    for i in range(2, math.floor(math.sqrt(x)) + 1):
        if x % i == 0:
            factors.append(i)
            if x//i != i:
                factors.append(x//i) 

    return factors

def is_consistent(x, divisors):
    # 1: The input could have a random incorrect value in it.
    # It's cheaper to do a linear scan of the divisors first even though
    # checking all factors does the same thing.
    for d in divisors:
        if x % d != 0:
            return False
    
    all_factors = generate_factors(x)
    # 2: The provided list of factors could be incomplete
    # This is another O(F^2) operation, whatever
    for f in all_factors:
        if f not in divisors:
            return False

    return True

for _i in range(0, t):
    _num_divisors = input()
    divisors = list(map(int, input().split()))
    x = min(divisors) * max(divisors)

    if is_consistent(x, divisors):
        print(x)
    else:
        print(-1)
