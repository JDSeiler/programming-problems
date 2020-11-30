#include <stdio.h>

/*
For a board where M is even and N is either even or odd, the board can be divided into M/2 rows where each row 
holds N dominos. Thus the total is M/2 * N or (M*N)/2. The same logic can be flipped to show the same result
when N is even.

When neither N or M is even, you can show that by greedily tiling the board you will always have exactly 1 cell 
left over (half a domino). Thus the answer is floor(M*N / 2). Integer division in C floors by default so that's
all you have to do.
*/

int main(void) {
    int m;
    int n;
    scanf("%d %d", &m, &n);

    printf("%d\n", (m*n) / 2);
    return 0;
}