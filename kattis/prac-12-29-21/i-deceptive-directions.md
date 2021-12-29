# Problem
Link: https://open.kattis.com/contests/zf5dyh/problems/deceptivedirections

You're given a grid, a starting point, and a set of directions (NSEW).

You know that every step is a transformation of the original path. Each
step has been replaced by a step in one of the three other directions.
So if the original step was North, it could now be South, East, or West. But
you aren't sure which. Your job is to mark all possible ending points
of the ORIGINAL path.

# Algorithm
This is a more complicated flood fill. For every direction we get, we know the
path did *not* go in that direction. So we explore in every OTHER direction 
and repeat the process. The problems statement says that the real path
is the shortest path to the treasure, so we do not allow doubling
back on already explored cells.

- For each cell in the frontier:
    - For each cell *not* in the given direction, add it to the frontier set
      if it is valid (not already explored and not an obstacle)
    - Repeat with next instruction

When we reach the last instruction the frontier is the set of all possible
locations for the treasure.

## Important Extra Detail
The above algorithm is almost right, but it needs an extra detail.
Your candidate results can only *optimal* paths. So if explore in a spiral,
the end result of that path is invalid because there's a shorter way.

To do this, you filter out candidate cells *during* the BFS by comparing
the depth of your BFS to a precomputed "optimal path length" for that cell.

So if you're considering exploring to a cell and you're at BFS depth 5 (you've
taken 5 steps) but the optimal distance is 3, you know this path is
incorrect and you shouldn't explore there.
