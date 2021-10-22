from bisect import bisect_right;

'''
bisect_right returns an insertion point `i`:

From the docs:
The returned insertion point i partitions the array a into two halves
so that 
all(val <= x for val in a[lo:i]) for the left side and
all(val > x for val in a[i:hi]) for the right side.
'''

t = int(input())

# Precomputation stuff

def compute_partials(upper_bound):
    curr = 0
    sums = []
    for i in range(1, upper_bound):
        next_val = curr + i;
        sums.append(next_val)
        curr = next_val
    return sums

UPPER = 3 * 10**5 # IDK I think this is big enough
partial_sums = compute_partials(UPPER)

def compute_rank(k):
    rank_upper_bound = bisect_right(partial_sums, k);
    rank_lower_bound = rank_upper_bound - 1;

    lower = partial_sums[rank_lower_bound];

    if (k-lower > 0):
        return rank_upper_bound + 1;
    else:
        return rank_lower_bound + 1;

def compute_indices(k, rank):
    # Rank is 1 indexed, sue me
    first_b_index = -2 - (rank-1)

    # If the rank is not set by the below if/else, the default SHOULD break everything 
    sum_of_previous_ranks = -9999999

    if (rank == 1):
        sum_of_previous_ranks = 0
    else:
        sum_of_previous_ranks = partial_sums[rank-2]

    index_in_rank = k - sum_of_previous_ranks
    second_b_index = index_in_rank * -1
    return (first_b_index, second_b_index)

def produce_string(indices, n):
    first, second = indices;
    # This could be horribly slow, IDK
    base = list('a' * n)
    base[first] = 'b'
    base[second] = 'b'
    return ''.join(base)


def solve_case():
    n, k = list(map(int, input().split()))
    rank = compute_rank(k)
    indices = compute_indices(k, rank)
    ans = produce_string(indices, n)
    print(ans)


for i in range(0, t):
    solve_case()

'''
1
aaabb

2
aabab
aabba

3
abaab
ababa
abbaa

4
baaab
baaba
babaa
bbaaa

We can use this fact to compute directly the indices of the b's.

We denote the position of the left-most `b` in the string it's "rank"

Let M be the number of strings in each rank:
M = [1, 2, 3, 4, 5, .... , N-1]

Let S be the array of partial sums of N
S  = [1, 3, 6, 10, 15]

We identify the rank of the k-th string by finding what values in S.
Take k=6 for an example

We can find the two brackets 6 lies between (lo <= 6 < hi): 6 and 10
(bisect gives as the index first and we look these values, up, so we
have the indices 2 and 3 as well)

So k is rank 3 or 4 (indices +1), but which?
We know the last element of rank 3 is the 6th string, and 6-6 = 0 so
k must be rank 3. If K was 7, then 7-6=1 (> 0) so we know it's rank 4.

Take k=1, we know it's between 1 and 3 and 1-1 = 0 so it's rank 1
But k=2, we know it's between 1 and 3 again, and 2-1 > 0, so it's rank 2


k=10 and we know it's between 10 and 15. 10-10 = 0 so it's rank 4 (index 3 + 1)

========
Once we know the rank of the string we have the index of the first b.

To find the other index, we want to know WHICH string this is inside the rank.

This is easy, it's just K minus the sum of all previous ranks.

So the 10th string is rank 4. The sum of previous ranks is 6, 10-6 = 4

The 1st string is rank 1. The sum of previous ranks is 0, 1-0 = 1
'''