# Problem
Link: https://open.kattis.com/contests/zf5dyh/problems/hyacinth

- You're given a tree of N nodes.
- You can assign two numbers to each node.
- Neighboring nodes can communicate if they share a number.
- A number is considered used if neighboring nodes share it.

Given a tree, your job is to output an assignment of numbers to nodes
which maximizes the number of used frequencies.

# Step 1: Rephrase the problem
Given a tree of N nodes, find a maximal edge coloring where each
edge may be colored with either one or two colors, and each node is
adjacent to at most 2 colors.

Note that this is not a "true" edge coloring because we allow adjacent edges
with the same color. But it is an edge coloring in the sense that we're assigning
colors to edges.

# Algorithm
The trivial case is a tree of two nodes, we handle it separately. Color the
only edge two unique colors of your choice.

In all other cases, the algorithm is as follows:
> If the root of the tree has only one child, re-root the tree by the original
  root's only child. This will cause the new root of the tree to have 2 children.
  If the root already has 2+ children this is unecessary.

Then, proceed as follows:

Begin at the root...

For each outgoing (towards the leaves) edge of the current node:
- If possible, give this edge a new unique color.
- If the current node has two colors already, pick an arbitrary color already
  in use by the current node for this edge.


- If the node has no outgoing edges (is a leaf):
  Assign it arbitrary colors until it has no free colors (if the algorithm
  is correct it will be assigned exactly one extra free color)

**Alternate way to handle 1-child roots**
> If all edges are colored and the current node still has a free color, repeat
  the process until the current node has no free colors. (This will really
  only ever happen when the root has 1 child node)

Repeat the process for each child node.

Once we have the edge coloring it is trivial to reconstruct the "frequencies"
that the problem requests. Since the assigned frequencies for each node are simply
the colors that it is adjacent to. Alternately, you may track these frequencies
as they're assigned (as the edges are colored) and output them at the end.

# Old Notes
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

This leads to an important observation:
**every internal node contributes exactly one unique edge color.**

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
