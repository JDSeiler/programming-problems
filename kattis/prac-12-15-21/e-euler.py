n = int(input())

if n == 0:
    print(1)
else:
    e = 1
    last_term = 1
    for i in range(1, n+1):
        # In general, you can derive via some algebra that 1/n! is equal to:
        # 1/n * 1/(n-1)!. Here, "last_term" is a memoization of 1/(n-1)!, but
        # you could do this recursively if you wanted.
        next_addend = (1/i) * last_term
        last_term = next_addend

        e += next_addend

    print(e)
