# Solution 1
Possible algorithm:
Pretend that you're a knight on a chess board, every time you land on a square
you write down a number on that cell. Every time you're picked up you increment
the number.

Look for a move to make, start with top-right and sweep counter clockwise. Make
the first valid move. If you ever get into a position where you don't have valid
moves to make, pick up the knight and make a row-by-row scan of the chess board
(starting at the top left) until you find an open cell.

Alternately, you could just run the heuristic for the knights walk (which runs in
linear time) till you get one. That'd work too.

I've tested this by hand up to 7x7 and it works.

4x4 is problematic unfortunately. Could just hardcode that case honestly.
# Solution 2
The second solution is far more practical. Simple traverse the board in alternating
diagonal stripes:
```
1 6 2
7 3 8
4 9 5
```
Start at the top left, mark that one. Then move over to the next diagonal not immediately
adjacent to the top left corner. Traverse it from top right to bottom left, marking the
cells consecutively. Continue this till you reach the bottom right corner. Then return
to the start and repeat the procedure with the rows not yet marked, just incrementing 
the numbers consecutively. For example, if the bottom right cell has 5, then the next
cell you mark back near the start will be 6.