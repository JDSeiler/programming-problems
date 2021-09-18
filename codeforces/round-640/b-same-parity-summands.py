t = int(input())

def solve_case():
    n, k = list(map(int, input().split()))

    # Trivially impossible
    if k > n:
        print("NO")
        return

    if n % 2 == 0:
        # Edge case when an even number is produced by odd summands
        if k == n:
            ans = [1 for i in range(0, k)]
            print("YES")
            helper_print_list(ans)
        # General cases down here
        elif k > n / 2:
            # This is getting out of hand
            if k % 2 == 0:
                handle_odd(n, k)
            else:
                print("NO")
                return
        else:
            handle_even(n, k)
    else:
        if k % 2 == 0:
            print("NO")
            return
        else:
            handle_odd(n, k)

def handle_even(n, k):
    ans = []
    unsquished_twos = k - 1

    remainder = (n - 2*unsquished_twos)
    ans.append(remainder)
    for _i in range(0, unsquished_twos):
        ans.append(2)

    print("YES")
    helper_print_list(ans)


def handle_odd(n, k):
    ans = []
    unsquished_ones = k - 1

    remainder = (n - unsquished_ones)
    ans.append(remainder)
    for _i in range(0, unsquished_ones):
        ans.append(1)

    print("YES")
    helper_print_list(ans)

def helper_print_list(l):
    for i in l:
        print(i, end=" ")
    print()

for i in range(0, t):
    solve_case()

'''
10 3
= 6 2 2

Obvservation 1: All even numbers can be written as the sum of any number of even numbers
Proof Sketch:
Given N = 2*j, and the constraint K write out j 2's, then combine 2's together until you have only k numbers total:

10, 3
10 = 2 2 2 2 2, combine 3 2's together to get 6 2 2


If j < k, it's impossible!
8 7
2 2 2 2. But 4 < 7 so this is impossible.

Observation 2: All odd numbers cannot be written as the sum of only even numbers.
This would imply the number is divisible by 2, which means it's even. A contradicition.
Therefore all odd numbers can only be written as the sum of other odd numbers, if it's possible at all.

Observation 3:
If the target is odd and K is even, it's impossible:

Odd + Odd is always even: 2n+1 + 2m+1 = 2(m + n + 1)
Even + Even is always even (this is obvious)
Therefore if the target is odd, and K is even, it's impossible.

So, knowing that k must be odd in solveable odd cases, we can construct a similar algorithm for odd numbers:

N, k
Write out N 1's, then reserve k-1 1's and combine the rest into a single number:

13 5
1 1 1 1 < reserve 4 1s
9 < the rest lumped together, aka (N - k - 1)

10, 6
5 1 1 1 1 1
'''