import math

def comb(n, k):
    return math.factorial(n) // (math.factorial(k) * math.factorial(n-k))

def count(haystack, needle):
    c = 0;
    for elem in haystack:
        if elem == needle:
            c = c + 1
        else:
            continue

    return c

def handle_case():
    _, k = map(int, input().split())
    # What is this, Lisp?
    bloggers = list(reversed(sorted(map(int, input().split()))))

    s = bloggers[:k]
    decider = s[-1]

    total = count(bloggers, decider)
    slice_only = count(s, decider)

    big_answer = comb(total, slice_only)
    real_answer = big_answer % ((10**9) + 7)
    # Solution isn't passing even though my algorithm is correct, something funky
    # with modulo? The official solution does modulo all over the place but that
    # doesn't seem to work for me.
    print(real_answer)

t = int(input())
for _ in range(0, t):
    handle_case()