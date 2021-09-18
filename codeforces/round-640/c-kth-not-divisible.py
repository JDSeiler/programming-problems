import math
t = int(input())

def solve_case():
    n, k = list(map(int, input().split()))
    if k < n:
        print(k)
    else:
        bracket = math.floor(k / (n-1))
        # The bracket math runs into trouble if it's a whole number
        # in that case it must be adjusted manually
        if k / (n-1) % 1 == 0:
            bracket = bracket - 1

        prior = (n-1) * (bracket-1) + (n-1)
        start = n * bracket
        to_add = k - prior
        print(start+to_add) 

for i in range(0, t):
    solve_case()

'''
Find the Kth number not divisible by N

Observation 1: All numbers smaller than N are candidates
Therefore if K < N, the answer is K

50, 30 => 30

Observation 2: All numbers between multiples of N are candidates
N, N+1, N+2 ... 2N, ... 3N, ... 4N

Observation 3: The AMOUNT of candidates between each multiple is N-1
Once you add N, that's the same as 2N (a multiple).

The bracket math doesn't work quite right if n is divisible by k-1.
In that case you've got to manually shift back 1 bracket because otherwise
you end up starting at the next multiple.

N, K s.t. K>N
floor(K / N-1) => the "bracket" the answer is in: call it B

N * B-1 + N-1 => the number of elements prior to this bracket: P
B + K-P = Answer?
'''