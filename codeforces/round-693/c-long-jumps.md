# Problem
https://codeforces.com/contest/1472/problem/C
# Solution
This is memoization. Or alternately a problem
of topological ordering.

# Memoization
You start building a graph where nodes
link to where they go. When they go off
edge of the array they just go to some
edge of value 0. You want the max sum
path that ends at the 0 node.

When a path reaches the 0 node, you go backwards
along the path and sum everything up and associate
it each node. If you reach a node you've already
visited you can just reuse its value.
# Topological Ordering
If you know the order to process the subproblems
then you can process everything using loops.

You can process them all backwards to get the answer!
