# Problem
Link: https://open.kattis.com/contests/zf5dyh/problems/hyacinth

- You're given a tree of N nodes.
- You can assign two numbers to each node.
- Neighboring nodes can communicate if they share a number.
- A number is considered used if neighboring nodes share it.

Given a tree, your job is to output an assignment of numbers to nodes
which maximizes the number of used frequencies.

# Alternate phrasing and details
This problem is edge coloring with an edge case and a condition.

The condition is that each node can be adjacent to at most 2 colors.

The edge case is a tree with only two nodes. In that case the optimal
strategy is to "double-color" the edge. By assigning the same pair
of numbers to both nodes. This is the only time double-coloring
is necessary. Since in a larger graph any double coloring forces
duplication on a nearby edge, so you're just "borrowing" a color
from another edge.

## Branching Factor
The ratio of colors to nodes depends on the branching factor of the tree.
For a line graph of n nodes, the number of colors is n-1 since you can uniquely
color each edge in the tree with no duplication. (the only exception, as 
previously noted, is a graph with 2 nodes in it).

If the branching factor is high, many edges get duplicate colors. Consider
some internal node J:
- J already has one number in use since it must communicate with its parent.
- Therefore J may choose one unused number to communicate with the first
  of its children. However, subsequent edges must reuse one of the numbers
  already present on J.

This leads to an important observation: every internal node contributes
exactly one unique edge color.

The exception to this pattern is the root of the tree. Since the root has
no parents, all colors it assigns are unique. The amount of unique numbers
it can contribute depends on how many edges it has.
- If the root has 1 edge, it can contribute 1 unique color. Even though
  you can double color the edge, it just forces duplication at the next level
  and so really you're only adding one unique color overall.
- If you have 2 or more edges, the root contributes exactly 2 colors.

Note, the alternative to double coloring is to select one number for communication
and then pick some random other number for the root node.

In the situation where the root has one node, double coloring is easier to reason
about because it also covers the situation where there are only two nodes. It
isn't a *worse* choice to double color, so you may as well. From an implementation
standpoint double coloring is really something you do with the only child node
of the root.

Another way to think about it is this:
- For leaf nodes, we know that we must use one of the colors of our parent.
- The other color choice is irrelevant because our parent is already using
  two colors and we have no children. So we choose whatever we want.
- For any tree, it's either a line graph or it isn't. If the tree isn't a line
  graph but the root has 1 child, we can always reroot the tree from some other
  node such that the root has more than 1 child, and the old root is now
  a leaf somewhere. Then the algorithm proceeds normally with no double coloring.

But, if the tree is a line-graph, this doesn't really make sense/a difference.
But that's ok because the line graph case is pretty simple. Additionally, we don't
*need* this extra observation to solve the problem for any tree.

## Number of Colors
So, for line graphs the # of colors is n-1, easy.

For a tree with I internal nodes, the number of colors is I + R. Where R is the
number of colors from the root of the tree. If the root has one child this is 1,
otherwise it is 2.

The 14 node example on the problem page lines up with this theory. The only unique
colors are:
- 4711
- 666
- 42
- 23
- 815
- 7

The formula predicts 6 colors. 4 from the 4 internal nodes and 2 from the root.

# Algorithm
For each node, we track the two numbers assigned to it.
We also maintain a global list of all used edge colors.

Begin at the root...

For each outgoing (towards the leaves) edge of the current node:
- If possible, give this edge a new unique color.
    - When a color is assigned, add that color to both adjacent nodes' list
- If the current node has two colors already, pick an arbitrary color already
  in use by the current node for this edge.
- If all edges are colored and the current node still has a free color, repeat
  the process until the current node has no free colors. (This will really
  only ever happen when the root has 1 child node)

IF the node has no outgoing edges (is a leaf):
Assign it arbitrary colors until it has no free colors (if the algorithm
is correct it will be assigned exactly one extra free color)

Repeat the process for each child node.