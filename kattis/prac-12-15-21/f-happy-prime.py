from math import ceil, sqrt

p = int(input())

def is_prime(n):
    if n == 1:
        return False

    for i in range(2, ceil(sqrt(n)) + 1):
        if n % i == 0:
            return False

    return True

def is_happy(n):
    seen = set()

    curr_num = n
    while not curr_num == 1:
        new_num = sum([int(d) * int(d) for d in str(curr_num)])
        if new_num in seen:
            return False
        else:
            seen.add(new_num)
            curr_num = new_num

    return True

def test(m):
    prime = is_prime(m)
    if prime:
        happy = is_happy(m)
        if happy:
            return "YES"
        else:
            return "NO"
    else:
        return "NO" 

for i in range(0, p):
    k, m = list(map(int, input().split()))
    is_happy_prime = test(m)

    print(f"{k} {m} {is_happy_prime}")
