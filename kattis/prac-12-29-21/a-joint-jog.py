from math import sqrt, pow

'''
Proof Sketch:
We have two lines: k and o, representing the paths of
Kari and Ola respectively.

We can always rotate and shift the plane such that one of the lines has its
start point at the origin and its end point ends on the Y axis. This
transformation does not change the relative distances between any pair of points
between the lines. Lets say we fix k.

With k fixed on the Y-axis, there are several broad cases:
1. o is parallel to k
2. o is moving towards k (but does not cross k)
3. o is moving away from k (but does not cross k)
4. o crosses k (moving either left to right or right to left)

If o is parallel to k, then the distance between any pair of points between the
lines does not change. We know this by the definition of being parallel.

Otherwise, if o is moving towards k, then the maximum distance must be the 
distance between the start of o and the start of k. Because o uniformly
moves towards k all other choices are strictly worse.

If o is moving away from k then the opposite is true, the end point of o
has to be the farthest away from any point on k.

In either of these cases it doesn't matter what side of k the line o is on
because you can just mirror the image without loss of generality.

For the final case, the distance between Kari and Ola is going to start at some value,
d1, then decrease (since they both run towards the point where the lines cross)
and then increases again to some value d2 where they're both at the end of their paths.
The maximum distance is then the larger of d1 and d2.

This operation of computing the distance between the start and end points and
then choosing the largest generalizes to all cases.
'''

def dist(p1, p2):
    x1, y1 = p1
    x2, y2 = p2
    return sqrt(pow(x1-x2, 2) + pow(y1-y2, 2))

coords = list(map(int, input().split()))
k_start = (coords[0], coords[1])
o_start = (coords[2], coords[3])
k_end = (coords[4], coords[5])
o_end = (coords[6], coords[7])


ans = max(dist(k_start, o_start), dist(k_end, o_end))

if ans == int(ans):
    print(int(ans))
else:
    print(ans)
