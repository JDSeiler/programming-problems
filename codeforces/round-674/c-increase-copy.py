'''
This solution is not working right now, there are a bunch of off-by-one
cases I can't figure out... Don't know what's wrong.

The solution to this is to find the fastest growing strategy:

One guess would be the following:
1 -> 1 1 -> 1 2 -> 1 2 2 -> 1 2 3

Which grows O(n^2), but it's still pretty slow.

A better strategy is to grow for G moves and then duplicate the largest element for C moves.
This is given by:
f(g,c) = (g+1)*(c+1)

f(1,1) = 4
1 -> 2 -> 2 2

If you inpsect the partial derivatives of the function f, you will find that f is maximized when g and c are the same.
Or rather when they both grow at the same rate. So if you do some algebra you can change f(g,c) into:
f(x) = x^2 + 2x + 1

The answer to the problem is based on the (positive) solution to f(x) = n, which can be calculated with some basic algebra.
x^2 + 2x + 1 -> (x+1)^2
(x+1)^2 = n
x+1 = sqrt(n)
x = sqrt(n)-1

If the answer has no fractional part, then the answer is 2x.
If the answer has a fractional part less than 1/2, the answer is 2*floor(x) + 1
If the answer has a fractional part greater than 1/2, the answer is 2*ceil(x)
'''
import math
t = int(input())

def diff(a,b,epsilon):
    return abs(a-b) <= epsilon

def solve_case():
    n = int(input())
    if n == 1:
        print(0)
        return
    
    solution = math.sqrt(n) - 1
    fract_part = solution - int(solution) # May be slightly more precise than using the % 1 trick

    # Perfect squares don't fit in the rounding logic
    if (diff(fract_part, 0, 0.000001)):
        print(2*math.floor(solution))

    if (fract_part < 0.5 or diff(fract_part, 0.5, 0.000001)):
        print(2*math.floor(solution) + 1)
    else:
        print(2*math.ceil(solution))

for i in range(0, t):
    solve_case()
