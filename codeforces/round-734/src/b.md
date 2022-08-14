Here is a restatement of the problem because the codeforces statement is hard to follow.

You are given an array `a` of length `n` and a number of colors 1 <= `k` <= `n`

You may choose to paint or not paint each element of `a` in any of the `k` colors, so 
long as you satisfy the following conditions:
1. All the numbers painted in the *same* color are unique.
2. All colors are used the same number of times (all colors paint the same number of cells)

Your job is to find a maximum coloring. That is, a coloring that obeys the above rules and
paints at least as many cells as any other valid coloring.

The output is a string which shows the coloring of each cell. 0 means uncolored, the digits
1 through k represent unique colors. Example

Input (n and k first as a pair, followed by a):
13 3
3 1 4 1 5 9 2 6 5 3 5 8 9 (a)

Output:
1 1 3 2 1 3 3 1 2 2 3 2 0

Strategy thoughts:

Idea 1: Is greedy optimal?

1. Tag each number with its original index in the array.
2. Sort everything by its value so that duplicates are colocated.
3. Put it all in a stack `S`

Suppose we have `k` colors:

While S isn't empty:
    Let the current color `c` := 1

    While we've not used `c` this round:
        Pop an element `n` off `S`
        Check if coloring `n` as `c` is valid
        If yes, mark down `n` as being colored `c` and break
        If no, move `n` to a special "uncolored" color so we can output it later.
    
    c := (c + 1) mod k << this line is off by one, but the idea is just wrap around to the first color

At the end, find the smallest color (the difference in size between all colors should be at most 1)
And move elements from colors that are too large to the "uncolored" color.

Objections:
O1: Why can we just discard colors if we can't immediately use them?
 A: This is not rigorous, but the intuition follows: Suppose that we are discarding a
    cell that some subsequent color could use. That would mean we have seen this number
    before but a subsequent color has not. Because the array is sorted, all duplicates are
    adjacent. And because we have seen this color before (and all colors are visited round-robin)
    this means that subsequent colors MUST have seen this number before. Therefore, if a number 
    is ever invalid for one color, it is invalid for all colors (precisely because we process cells
    in order of their numeric value) 

O2: Can a greedy choice be suboptimal? What if painting a cell some color actually "locks off"
    some better choice for another color?
 A: This is not rigorous either, but the intuition follows: The thing that determines what cells
    you are or are not allowed to color is uniqueness. Because we process cells in sorted order,
    each new number encountered is either unique to the current color, or it has already been 
    by all colors and the choice to not paint it is forced. So, all that is important is that all
    colors get equal access to painting cells, which is accomplished by assigning them cells in 
    a round-robin fashion.


