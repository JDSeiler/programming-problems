# Idea
From messing around with it on a whiteboard, I think
the solution may be always picking the smallest move
that connects a sheep to another sheep. But it gets
really tricky because you could get a situation like:
```
**...*
// Moving to the sheep on the right.. shortest move on the board
*.*..*
// Oop! Move of 1! Bingo!
**...*
// Repeat...
```
This example is super contrived because obviously the
correct choice is to move the right sheep towards the
group of two, and not move the left sheep at all. But
you could imagine other situations where this might
pop up. 

Would have to come up with some strategy to address this.
There is meaning in saying the "shortest possible move"
but you have to be precise with what that means and think
carefully about how to break ties. I think a key idea may
be to only allow smaller groups to go towards larger groups.

The input size is bounded at a million (total cells), so
I think it'd be possible to simulate?

# Big-O
I think the worst case for my proposed solution is an alternating pattern:
```
*.*.*.*.*.*.*. <etc>
```
Because the optimal is (smaller example):
```
*.*.*.*.*.
*..**.*.*.
*..***..*.
..****..*.
..*****...
```
The first two sheep have to make 1 move, the next two make 2 moves.
For N sheep you will have roughly N/4 pairs (N/2 sheep total, then
-1 center sheep, divide by 2 again to get the pairs).

The total number of moves is the sum of the first floor(N/4) even
numbers. Which I think is equivalent (asymptotically) to N^2. Since
it's just like a triangular summation (1+2+3+...+N = O(N^2)).

So you may have to be smarter than just simulating it.
